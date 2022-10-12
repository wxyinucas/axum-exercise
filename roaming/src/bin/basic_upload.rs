use askama::Template;
use axum::extract::{ContentLengthLimit, Multipart};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::{routing, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", routing::get(upload_page).post(upload_action));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10;

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadPage;

async fn upload_page() -> Result<Html<String>, String> {
    let page = UploadPage;
    let html = page.render().map_err(|e| e.to_string())?;

    Ok(Html(html))
}

async fn upload_action(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
) -> Result<(HeaderMap, String), String> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();

        // save files in 'my_axum/'
        tokio::fs::write(&filename, &data)
            .await
            .map_err(|e| e.to_string())?;

        let message = format!("UPLOAD FILE >> name: {}, size: {}", filename, data.len());
        return res(message).await;
    }

    Err("No file uploaded!".to_string())
}

async fn res(message: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().unwrap(),
    );
    Ok((headers, message))
}
