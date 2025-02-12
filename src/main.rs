use hmac::{Hmac, Mac};
use poem::{
    middleware::{CatchPanic, Cors, Tracing},
    EndpointExt, Route,
};
use poem_api::{api::Api, db::load_db, env::load_env};
use poem_openapi::OpenApiService;
use sha2::Sha256;
use shuttle_poem::ShuttlePoem;

pub type ServerKey = Hmac<Sha256>;

#[shuttle_runtime::main]
async fn main() -> ShuttlePoem<impl poem::Endpoint> {
    tracing_subscriber::fmt().try_init().ok();
    let env = load_env("./.env.example")?;

    let server_key = env
        .get("APP_SECRET")
        .expect("`APP_SECRET` is not set on Env");

    let db = load_db().await?;

    let server_key =
        Hmac::<Sha256>::new_from_slice(server_key.as_bytes()).expect("Server Key Invalid");

    let item_service = OpenApiService::new(Api, "Items API", "1.0");

    let ui = item_service.swagger_ui();

    let app = Route::new()
        .nest("/", item_service.data(db).data(server_key).with(Tracing))
        .nest("/docs", ui)
        .with(CatchPanic::new())
        .with(Cors::new());

    Ok(app.into())
}
