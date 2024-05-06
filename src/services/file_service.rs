use crate::models::loggaroo::{File, LogMessage};
use crate::stores;
use crate::utils::shared_state::SharedState;
use axum::extract::Multipart;
use axum::http::StatusCode;
use regex::Regex;
use std::io::Read;
use tar::Archive;
use time::macros::format_description;
use time::PrimitiveDateTime;
use tracing::{info, warn};
use uuid::Uuid;
use xz::read::XzDecoder;

fn internal_error<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message.into())
}

fn bad_request<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, message.into())
}

pub async fn extract_zip(
    mut multipart: Multipart,
    session_id: String,
    state: SharedState,
) -> Result<(), (StatusCode, String)> {
    info!("uploading files for session: {}", session_id.clone());

    let uuid = Uuid::parse_str(&session_id).unwrap();

    let mut all_messages: Vec<LogMessage> = vec![];
    let mut files: Vec<File> = vec![];

    loop {
        let field = match multipart
            .next_field()
            .await
            .map_err(|_| internal_error("Encountered error while parsing multipart file"))?
        {
            None => break,
            Some(field) => field,
        };

        let data = field
            .bytes()
            .await
            .map_err(|_| internal_error("Encountered error while receiving data"))?;

        let tar = XzDecoder::new(data.as_ref());
        let mut archive = Archive::new(tar);

        let entries = archive
            .entries()
            .map_err(|_| internal_error("Failed to get entries of archive"))?;

        for entry in entries {
            match entry {
                Ok(mut entry) => {
                    if entry.path().unwrap().extension().unwrap().to_str().unwrap() != "txt" {
                        continue;
                    }
                    
                    let name = entry
                        .path()
                        .iter()
                        .filter_map(|p| p.file_name())
                        .filter_map(|n| n.to_str())
                        .map(|n| n.to_string())
                        .next();

                    let name = name
                        .ok_or_else(|| internal_error("Couldn't read name of entry in archive"))?;

                    files.push(File {
                        session_id: uuid,
                        file_name: name.clone(),
                        hash: "".to_string(),
                        chunk_count: 0,
                        uploaded_chunk_count: 0,
                    });

                    info!("Starting to read content from file {}", name);

                    let mut content = String::with_capacity(entry.size() as usize);
                    let _ = entry.read_to_string(&mut content).map_err(|_| {
                        internal_error("Failed to read content of archive entry to string")
                    })?;

                    info!("Finished reading content");

                    info!("Started parsing");

                    let mut messages: Vec<LogMessage> = vec![];
                    for (index, line) in content.lines().enumerate() {
                        match parse_line(uuid, name.clone(), index, line, &state.message_regex) {
                            None => match messages.last_mut() {
                                None => warn!("Invalid line without previous line!"),
                                Some(message) => message.content.push_str(line),
                            },
                            Some(message) => messages.push(message),
                        };
                    }

                    all_messages.append(&mut messages);

                    info!("Finished parsing");
                }
                Err(_) => {
                    return Err(bad_request("Invalid file found"));
                }
            }
        }
    }

    println!("{:?}", files);

    info!("Started writing to DB");

    stores::file_store::store_files(files, &state.pool).await;
    stores::file_store::store_messages(all_messages, &state.pool).await;

    info!("Finished writing to DB");

    Ok(())
}

fn parse_line(
    session_id: Uuid,
    file_name: String,
    index: usize,
    line: &str,
    message_regex: &Regex,
) -> Option<LogMessage> {
    let result = message_regex.captures(line).map(|captures| {
        let mut captures = captures
            .iter()
            .skip(1)
            .map(|capture| capture.map(|match_| match_.as_str().trim().to_string()));

        let date_format =
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second],[subsecond]");
        let date =
            PrimitiveDateTime::parse(&captures.next().unwrap().unwrap(), date_format).unwrap();

        LogMessage {
            session_id,
            file_name,
            entry_nr: index as i32,
            creation_date: date,
            classification: captures.next().unwrap().unwrap().to_lowercase(),
            service_ip: captures.next().unwrap(),
            user_id: captures.next().unwrap(),
            user_session_id: captures.next().unwrap(),
            java_class: captures.next().unwrap().unwrap(),
            content: captures.next().unwrap().unwrap(),
            sql_raw: None,
            sql_data: None,
        }
    });

    result
}
