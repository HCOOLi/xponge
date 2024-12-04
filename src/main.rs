use axum::{response::Html, routing::{get,post}, Router};
use tower_http::services::ServeDir;
use tokio::fs;
use std::path::Path;
use tokio::io::AsyncReadExt;
use serde::Deserialize;


#[derive(Deserialize)]
struct MyData {
    message: String,
    email: String,
}


#[tokio::main]
async fn main() {

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(index_handler))
        .route("/contact", get(contact_handler))
        .route("/projects", get(projects_handler))
        // .route("/email", post(email_handler))
        .nest_service("/assets", ServeDir::new("assets"));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<String> {
    let file_path = Path::new("index.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}

async fn contact_handler() -> Html<String> {
    let file_path = Path::new("contact.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}

async fn projects_handler() -> Html<String> {
    let file_path = Path::new("projects.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}


// async fn email_handler(headers: HeaderMap, form: Form<MyData>) -> Html<String> {
//     println!("Headers: {:?}", headers);
//     println!("Received message: {}", form.message);
//     println!("Received email: {}", form.email);
//     Html(String::from("Email received successfully"))
// }