use std::{fs::File, io::Write, path::Path};

use axum::{extract::Multipart, http::StatusCode};
use futures_util::Stream;

pub async fn save_file(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    while let Some(mut field) = multipart.next_field().await.map_err(|error| {
        tracing::error!("Error getting next field: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })? {
        let name = field
            .name()
            .map(ToString::to_string)
            .unwrap_or("name".to_owned());
        let file_name = field
            .file_name()
            .map(ToString::to_string)
            .unwrap_or("file_name".to_owned());
        let Some(file_type) = field.content_type().map(ToString::to_string) else {
            tracing::info!("we don't have a content type :(");
            break;
        };

        let file_extension = match file_type.as_str() {
            "image/png" => "png",
            _ => {
                tracing::error!("got a file extension we don't know about");
                return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            }
        };

        // let data = field.bytes().await.map_err(|error| {
        //     tracing::error!("Error get field bytes: {error}");
        //     StatusCode::INTERNAL_SERVER_ERROR
        // })?;

        // inside this one field we need to get the chunks until there are no more

        let mut file = File::create(&format!("{name}.{file_extension}")).map_err(|error| {
            tracing::error!("error opening file for writing: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        loop {
            let Some(data) = field.chunk().await.map_err(|error| {
                tracing::error!("Error getting chunk: {error}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            else {
                tracing::info!("no more chunks");
                break;
            };

            tracing::info!("processing field in multipart");
            tracing::info!(
                "name: {name} - file_name: {file_name} - data: {} - content type: {file_type}",
                data.len()
            );

            file.write_all(&data).map_err(|error| {
                tracing::error!("Error writing chunk to file: {error}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
    }

    Ok(StatusCode::OK)
}
