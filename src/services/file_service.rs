use axum::extract::Multipart;
use axum::http::StatusCode;
use std::io::Read;
use tar::Archive;
use xz::read::XzDecoder;

fn internal_error<S: Into<String>>(message: S) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message.into())
}

pub async fn extract_zip(mut multipart: Multipart) -> Result<(), (StatusCode, String)> {
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

        for mut entry in entries.flatten() {
            let name = entry
                .path()
                .iter()
                .filter_map(|p| p.file_name())
                .filter_map(|n| n.to_str())
                .map(|n| n.to_string())
                .next();

            let name =
                name.ok_or_else(|| internal_error("Couldn't read name of entry in archive"))?;

            let mut content = String::with_capacity(entry.size() as usize);
            let _ = entry
                .read_to_string(&mut content)
                .map_err(|_| internal_error("Failed to read content of archive entry to string"))?;

            // TODO TMP
            println!("{} - {} Bytes", name, content.len());
        }
    }
    Ok(())
}
