use axum::extract::Multipart;
use axum::http::StatusCode;
use std::io::Read;
use tar::Archive;
use tracing::info;
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

                    let mut content = String::with_capacity(entry.size() as usize);
                    let _ = entry.read_to_string(&mut content).map_err(|_| {
                        internal_error("Failed to read content of archive entry to string")
                    })?;

                    // TODO TMP
                    println!("{} - {} Bytes", name, content.len());
                }
                Err(_) => {
                    return Err(bad_request("Invalid file found"));
                }
            }
        }
    }
    Ok(())
}
