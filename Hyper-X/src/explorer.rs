use actix_web::{get, web, App, HttpServer, Responder};
use crate::storage::Blockchain;
use std::sync::{Arc, Mutex};

#[get("/blocks")]
async fn get_blocks(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    web::Json(&blockchain.ledger)
}

#[get("/latest_block")]
async fn latest_block(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    web::Json(blockchain.ledger.back())
}

pub async fn start_explorer(blockchain: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(get_blocks)
            .service(latest_block)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
