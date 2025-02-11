use jwt::SignWithKey;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    OpenApi, Tags,
};

use crate::{
    auth::{AuthUser, Authenticate, ServerKey},
    db::{save_db, Db},
    models::Item,
    requests::{ItemRequest, LoginRequest},
    responses::DeleteItemResponse,
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
    ) -> Result<PlainText<String>> {
        let user = AuthUser {
            username: req.0.username,
        }
        .sign_with_key(server_key.0)
        .map_err(InternalServerError)?;
        Ok(PlainText(user))
    }

    #[oai(path = "/items", method = "get", tag = "Group::Items")]
    async fn get_items(&self, db: Data<&Db>) -> Result<Json<Vec<Item>>> {
        let db = db.lock().await;
        Ok(Json(db.items.clone()))
    }

    #[oai(path = "/items", method = "post", tag = "Group::Items")]
    async fn add_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        req: Json<ItemRequest>,
    ) -> Result<Json<Item>> {
        let mut db = db.lock().await;
        let new_item = Item {
            id: db.last_id + 1,
            name: req.name.clone(),
        };
        db.last_id += 1;
        db.items.push(new_item.clone());

        save_db(&db).await?;
        Ok(Json(new_item))
    }

    #[oai(path = "/items/:id", method = "get", tag = "Group::Items")]
    async fn get_item(&self, db: Data<&Db>, id: Path<u32>) -> Result<Json<Item>> {
        let db = db.lock().await;
        db.items
            .iter()
            .find(|item| item.id == *id)
            .cloned()
            .map(Json)
            .ok_or_else(|| Error::from_status(StatusCode::NOT_FOUND))
    }

    #[oai(path = "/items/:id", method = "put", tag = "Group::Items")]
    async fn update_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
        req: Json<ItemRequest>,
    ) -> Result<Json<Item>> {
        let mut db = db.lock().await;

        let updated_item = {
            if let Some(item) = db.items.iter_mut().find(|item| item.id == *id) {
                item.name = req.name.clone();
                Some(item.clone())
            } else {
                None
            }
        };

        if let Some(item) = updated_item {
            save_db(&db).await?;
            Ok(Json(item))
        } else {
            Err(Error::from_status(StatusCode::NOT_FOUND))
        }
    }

    #[oai(path = "/items/:id", method = "delete", tag = "Group::Items")]
    async fn delete_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
    ) -> Result<Json<DeleteItemResponse>> {
        let mut db = db.lock().await;
        if let Some(pos) = db.items.iter().position(|item| item.id == *id) {
            db.items.remove(pos);

            save_db(&db).await?;

            return Ok(Json(DeleteItemResponse::default()));
        }
        Err(Error::from_status(StatusCode::NOT_FOUND))
    }
}
