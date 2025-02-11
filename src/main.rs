use hmac::{Hmac, Mac};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_api::{api::Api, db::load_db, env::load_env};
use poem_openapi::OpenApiService;
use sha2::Sha256;

pub type ServerKey = Hmac<Sha256>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let env = load_env("./.env.example")?;

    let host = env.get("HOST").expect("`HOST` is not set on Env");
    let port = env.get("PORT").expect("`PORT` is not set on Env");
    let server_key = env
        .get("APP_SECRET")
        .expect("`APP_SECRET` is not set on Env");

    let address = format!("{}:{}", host, port);

    let db = load_db().await?;

    let server_key =
        Hmac::<Sha256>::new_from_slice(server_key.as_bytes()).expect("Server Key Invalid");

    let item_service = OpenApiService::new(Api, "Items API", "1.0");

    let ui = item_service.swagger_ui();

    let app = Route::new()
        .nest("/", item_service.data(db).data(server_key))
        .nest("/docs", ui)
        .with(Cors::new());

    Server::new(TcpListener::bind(address)).run(app).await?;
    Ok(())
}
