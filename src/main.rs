use axum::{
    extract::Multipart,
    routing::post,
    http::StatusCode,
    Router,
};
use hyper::Server;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("uploaded_file").to_string();
        let data = field.bytes().await.unwrap();

        let mut file = File::create(format!("uploads/{}", file_name))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        file.write_all(&data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok("File uploaded successfully!".to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload_file));

    let addr = "0.0.0.0:3000".parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
