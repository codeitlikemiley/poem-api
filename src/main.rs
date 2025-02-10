use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_api::{api::Api, db::load_db};
use poem_openapi::OpenApiService;
use sha2::Sha256;

pub type ServerKey = Hmac<Sha256>;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let secret_binding = std::env::var("APP_SECRET").unwrap_or_else(|_| "secret".to_string());
    let server_key = secret_binding.as_bytes();

    let address = format!("{}:{}", host, port);

    let db = match load_db().await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error loading database: {}", err);
            std::process::exit(1);
        }
    };

    let server_key = Hmac::<Sha256>::new_from_slice(server_key).expect("valid server key");

    let item_service = OpenApiService::new(Api, "Items API", "1.0");

    let ui = item_service.swagger_ui();

    let app = Route::new()
        .nest("/", item_service.data(db).data(server_key))
        .nest("/docs", ui)
        .with(Cors::new());

    Server::new(TcpListener::bind(address)).run(app).await?;
    Ok(())
}
