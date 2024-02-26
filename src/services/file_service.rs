use crate::types::tmp_message::TmpMessage;
use axum::extract::Multipart;
use axum::http::StatusCode;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use regex::{Captures, Regex};
use std::io::Read;
use sqlx::types::time::Date;
use tar::Archive;
use tracing::info;
use xz::read::XzDecoder;
use time::macros::format_description;
use tracing::debug;

fn internal_error<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message.into())
}

fn bad_request<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, message.into())
}

pub async fn extract_zip(
    mut multipart: Multipart,
    message_regex: &Regex,
    session_id: String,
) -> Result<(), (StatusCode, String)> {
    info!("uploading files for session: {}", session_id);

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
                    let name = entry
                        .path()
                        .iter()
                        .filter_map(|p| p.file_name())
                        .filter_map(|n| n.to_str())
                        .map(|n| n.to_string())
                        .next();

                    let name = name
                        .ok_or_else(|| internal_error("Couldn't read name of entry in archive"))?;

                    debug!("Starting to read content from file {}", name);

                    let mut content = String::with_capacity(entry.size() as usize);
                    let _ = entry.read_to_string(&mut content).map_err(|_| {
                        internal_error("Failed to read content of archive entry to string")
                    })?;

                    debug!("Finished reading content");

                    debug!("Started parsing");

                    let messages: Vec<_> = content
                        .lines()
                        .par_bridge()
                        .map(|line| parse_line(line, message_regex))
                        .collect();

                    debug!("Finished parsing");


                }
                Err(_) => {
                    return Err(bad_request("Invalid file found"));
                }
            }
        }
    }
    Ok(())
}

fn parse_line(line: &str, message_regex: &Regex) -> Option<TmpMessage> {
    message_regex.captures(line).map(|captures| {
        let mut captures = captures.iter()
            .skip(1)
            .map(|capture| capture.map(|match_| match_.as_str().trim().to_string()));


        let date_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second],[subsecond]");
        let date = Date::parse(&captures.next().unwrap().unwrap(), date_format).unwrap();

        TmpMessage::new(
            date,
            captures.next().unwrap().unwrap(),
            captures.next().unwrap(),
            captures.next().unwrap(),
            captures.next().unwrap(),
            captures.next().unwrap().unwrap(),
            captures.next().unwrap().unwrap(),
        )
    })
}
