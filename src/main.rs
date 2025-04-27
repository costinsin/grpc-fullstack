mod middleware;
mod services;

use log::info;
use tonic::transport::Server;

use chrono::Local;
use services::calculator::calculator_server::CalculatorServer;
use std::io::Write;
use tonic_middleware::MiddlewareLayer;

const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("reflection_descriptor_set");

fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.6f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let addr = "[::1]:50051".parse()?;
    info!("Listening on {}", addr);
    // Load the file descriptor set for reflection
    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Create servers for every proto service
    let calculator_server =
        CalculatorServer::new(services::calculator::CalculatorService::default());

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tonic_web::GrpcWebLayer::new())
        .layer(MiddlewareLayer::new(
            middleware::logging::LoggingMiddleware::default(),
        ))
        .add_service(reflection_server)
        .add_service(calculator_server)
        .serve(addr)
        .await?;

    Ok(())
}
