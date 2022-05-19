use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use async_minecraft_ping::ConnectionConfig;
use serde::Serialize;

use crate::error::MCError;

#[derive(Serialize)]
pub struct Response {
    pub max: u32,
    pub online: u32,
    pub version: String,
    pub description: String,
}

#[get("/last")]
async fn last(session: Session) -> Result<impl Responder, MCError> {
    if let Ok(Some(ip)) = session.get::<String>("ip") {
        return Ok(HttpResponse::Ok().body(ip));
    }

    Ok(HttpResponse::Ok().body(""))
}

#[get("/status/{ip_address}")]
async fn status(session: Session, path: web::Path<String>) -> Result<impl Responder, MCError> {
    let ip = path.into_inner();
    println!("{}", ip);

    let config = ConnectionConfig::build(ip.clone());

    let connection = config.connect().await.map_err(|_| MCError::ServerOffline)?;
    let status = connection
        .status()
        .await
        .map_err(|_| MCError::ServerOffline)?
        .status;

    let description = match status.description {
        async_minecraft_ping::ServerDescription::Plain(v) => v,
        async_minecraft_ping::ServerDescription::Object { text } => text,
    }
    .trim()
    .to_owned();

    let resp = Response {
        max: status.players.max,
        online: status.players.online,
        version: status.version.name,
        description,
    };

    let _ = session.insert("ip", ip);

    Ok(HttpResponse::Ok().json(resp))
}
