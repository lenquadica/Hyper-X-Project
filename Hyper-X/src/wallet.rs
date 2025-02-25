use actix_web::{web, App, HttpServer, Responder, get, post};
use crate::transactions::Transaction;
use std::sync::{Arc, Mutex};

#[post("/send")]
async fn send_transaction(
    data: web::Data<Arc<Mutex<Vec<Transaction>>>>,
    txn: web::Json<Transaction>,
) -> impl Responder {
    let mut transactions = data.lock().unwrap();
    transactions.push(txn.into_inner());
    "✅ Transaction Sent!"
}

#[get("/balance/{user}")]
async fn get_balance(
    data: web::Data<Arc<Mutex<Vec<Transaction>>>>,
    user: web::Path<String>,
) -> impl Responder {
    let transactions = data.lock().unwrap();
    let balance: i64 = transactions.iter().fold(0, |acc, txn| {
        if txn.sender == *user {
            acc - txn.amount as i64
        } else if txn.receiver == *user {
            acc + txn.amount as i64
        } else {
            acc
        }
    });

    format!("💰 {}'s Balance: {}", user, balance)
}

pub async fn start_wallet_api(transactions: Arc<Mutex<Vec<Transaction>>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(transactions.clone()))
            .service(send_transaction)
            .service(get_balance)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
