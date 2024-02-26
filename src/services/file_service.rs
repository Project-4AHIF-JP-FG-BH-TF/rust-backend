use axum::extract::Multipart;
use axum::http::StatusCode;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use tar::Archive;
use tracing::info;
use xz::read::XzDecoder;

fn internal_error<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message.into())
}

fn bad_request<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, message.into())
}

pub async fn extract_zip(mut multipart: Multipart) -> Result<(), (StatusCode, String)> {
    let mut thread_handles = vec![];

    loop {
        let field = match multipart
            .next_field()
            .await
            .map_err(|_| internal_error("Encountered error while parsing multipart file"))?
        {
            None => break,
            Some(field) => field,
        };

        let bytes = field
            .bytes()
            .await
            .map_err(|_| internal_error("Encountered error while receiving data"))?;

        let handle = tokio::spawn(async move {
            let tar = XzDecoder::new(bytes.as_ref());
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

                        let name = name.ok_or_else(|| {
                            internal_error("Couldn't read name of entry in archive")
                        })?;

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

            Ok(())
        });

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle
            .await
            .map_err(|_| internal_error("Encountered error while receiving data"))??;
    }
    Ok(())
}
