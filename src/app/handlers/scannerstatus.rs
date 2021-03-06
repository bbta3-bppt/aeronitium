use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::liveaction;


#[derive(Deserialize)]
pub struct DtcScannerStatus {
    crs: String,
    scanner_number: u8
}

#[post("/scannerstatus")]
pub async fn get_status(
    payload: web::Json<DtcScannerStatus>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = liveaction::lookat(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.scanner_number
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
