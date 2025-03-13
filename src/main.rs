use axum::{
    extract::Multipart,
    routing::post,
    Router,
};
use std::{net::SocketAddr, io::Error};
use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener};

async fn upload_file(mut multipart: Multipart) -> Result<String, Error> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("uploaded_file").to_string();
        let data = field.bytes().await.unwrap();

        let mut file = File::create(format!("uploads/{}", file_name)).await?;
        file.write_all(&data).await?;
    }
    Ok("File uploaded successfully!".to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
