use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, App, HttpServer};
use dotenv::dotenv;
use std::env;

use crate::endpoints::{last, status};

mod endpoints;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("It's Minecraft time");

    // Load .ENV file
    dotenv().ok();

    // Get host, port and postgres url from env
    let host = env::var("HOST").expect("HOST not set!");
    let port = env::var("PORT").expect("PORT not set!");

    HttpServer::new(move || {
        App::new()
            .wrap(create_cookie_session())
            .service(status)
            .service(last)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

fn create_cookie_session() -> SessionMiddleware<CookieSessionStore> {
    // Load .ENV file
    dotenv().ok();

    // Get session key
    let session_key = env::var("SESSION_KEY").expect("SESSION_KEY not set!");
    let session_key = Key::from(session_key.as_bytes());

    SessionMiddleware::new(CookieSessionStore::default(), session_key)
}
