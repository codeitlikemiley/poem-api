use jwt::SignWithKey;
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi, Tags};

use crate::{
    auth::{AuthUser, Authenticate, ServerKey},
    db::{save_db, Db},
    models::Item,
    requests::{ItemRequest, LoginRequest},
    responses::{Responses, Return, Schema},
};

#[derive(Tags)]
enum Group {
    Auth,
    Items,
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/login", method = "post", tag = "Group::Auth")]
    async fn login(
        &self,
        server_key: Data<&ServerKey>,
        req: Json<LoginRequest>,
    ) -> Responses<String> {
        let user = AuthUser {
            username: req.0.username,
        }
        .sign_with_key(server_key.0);

        match user {
            Ok(resource) => Return::ok(resource),
            Err(e) => Return::internal_error(e.to_string()),
        }
    }

    #[oai(path = "/items", method = "get", tag = "Group::Items")]
    async fn get_items(&self, db: Data<&Db>) -> Responses<Vec<Item>> {
        let db = db.lock().await;
        Return::success(db.items.clone())
    }

    #[oai(path = "/items", method = "post", tag = "Group::Items")]
    async fn add_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        req: Json<ItemRequest>,
    ) -> Responses<Item> {
        let mut db = db.lock().await;

        let new_item = Item {
            id: db.last_id + 1,
            name: req.name.clone(),
        };

        db.last_id += 1;
        db.items.push(new_item.clone());

        match save_db(&db).await {
            Ok(_) => Return::created(new_item),
            Err(e) => Return::internal_error(e.to_string()),
        }
    }

    #[oai(path = "/items/:id", method = "get", tag = "Group::Items")]
    async fn get_item(&self, db: Data<&Db>, id: Path<u32>) -> Responses<Item> {
        let db = db.lock().await;

        match db.items.iter().find(|item| item.id == *id).cloned() {
            Some(item) => Return::success(item),
            None => Return::not_found(),
        }
    }

    #[oai(path = "/items/:id", method = "put", tag = "Group::Items")]
    async fn update_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
        req: Json<ItemRequest>,
    ) -> Responses<Item> {
        let mut db = db.lock().await;

        match db.items.iter_mut().find(|item| item.id == *id) {
            Some(item) => {
                item.name = req.name.clone();
                let updated_item = item.clone();

                match save_db(&db).await {
                    Ok(_) => Return::success(updated_item),
                    Err(e) => Return::internal_error(e.to_string()),
                }
            }
            None => Responses::NotFound(Json(Schema::not_found())),
        }
    }

    #[oai(path = "/items/:id", method = "delete", tag = "Group::Items")]
    async fn delete_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
    ) -> Responses<String> {
        let mut db = db.lock().await;

        match db.items.iter().position(|item| item.id == *id) {
            Some(pos) => {
                db.items.remove(pos);

                match save_db(&db).await {
                    Ok(_) => Return::ok("Item deleted successfully".to_string()),
                    Err(e) => Return::internal_error(e.to_string()),
                }
            }
            None => Return::not_found(),
        }
    }
}
