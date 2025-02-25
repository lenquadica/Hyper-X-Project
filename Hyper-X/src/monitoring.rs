use prometheus::{Opts, Registry, IntCounter, Encoder, TextEncoder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

pub struct Monitoring {
    pub block_count: IntCounter,
    pub transaction_count: IntCounter,
}

impl Monitoring {
    pub fn new(registry: &Registry) -> Self {
        let block_count = IntCounter::new("hyperx_blocks", "Total Blocks Produced").unwrap();
        let transaction_count = IntCounter::new("hyperx_transactions", "Total Transactions").unwrap();

        registry.register(Box::new(block_count.clone())).unwrap();
        registry.register(Box::new(transaction_count.clone())).unwrap();

        Self { block_count, transaction_count }
    }
}

#[get("/metrics")]
async fn metrics(data: web::Data<Arc<Mutex<Monitoring>>>) -> impl Responder {
    let monitoring = data.lock().unwrap();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok().body(buffer)
}

pub async fn start_monitoring_server(monitoring: Arc<Mutex<Monitoring>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(monitoring.clone()))
            .service(metrics)
    })
    .bind("127.0.0.1:9091")?
    .run()
    .await
}
