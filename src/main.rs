#[warn(unused_imports)]
use axum::{
    extract::Multipart,
    response::Html,
    routing::get,
    Router,
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tera::{Context, Tera};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create the directory for uploaded files if it doesn't exist
    create_dir_all("files").expect("Failed to create directory for uploaded files");

    let app = Router::new().route("/", get(index).post(upload));

    let listener = TcpListener::bind("localhost:5050").await.unwrap();
    let address = listener.local_addr().unwrap();

    println!("Server running on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error initializing Tera: {}", e);
            return Html("Error loading templates".to_string());
        }
    };

    let context = Context::new();
    match tera.render("index.html", &context) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            Html("Error rendering template".to_string())
        }
    }
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("failed to extract field")
    {
        if field.name().unwrap_or_default() != "fileupload" {
            continue;
        }

        println!("Got file");

        if let Some(file_name) = field.file_name() {
            let file_path = format!("files/{}", file_name);

            let data = field.bytes().await.unwrap();

            let mut file_handle = File::create(&file_path).expect("failed to open file handle");

            file_handle.write_all(&data).expect("failed to write data");

            println!("File saved to: {}", file_path);
        } else {
            eprintln!("No file name provided");
        }
    }
}
