use jwt::SignWithKey;
use poem::{web::Data, Result};
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
    responses::{
        create_item, fetch_items, find_item, login, modify_item,
        remove_item::{self, DeletedMessage},
    },
};
use validator::Validate;

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
    ) -> Result<login::Response, login::Error> {
        use login::{
            Error::{InternalError, ValidationErrors},
            Response,
        };

        if let Err(e) = req.0.validate() {
            return Err(ValidationErrors(Json(e.into())));
        }

        let user = AuthUser {
            username: req.0.username,
        }
        .sign_with_key(server_key.0)
        .map_err(|_| InternalError)?;

        Ok(Response::Ok(PlainText(user)))
    }

    #[oai(path = "/items", method = "get", tag = "Group::Items")]
    async fn get_items(&self, db: Data<&Db>) -> Result<fetch_items::Response, fetch_items::Error> {
        use fetch_items::Response;

        let db = db.lock().await;

        Ok(Response::Ok(Json(db.items.clone())))
    }

    #[oai(path = "/items", method = "post", tag = "Group::Items")]
    async fn add_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        req: Json<ItemRequest>,
    ) -> Result<create_item::Response, create_item::Error> {
        use create_item::{
            Error::{InternalError, ValidationErrors},
            Response,
        };

        if let Err(e) = req.0.validate() {
            return Err(ValidationErrors(Json(e.into())));
        }

        let mut db = db.lock().await;

        let new_item = Item {
            id: db.last_id + 1,
            name: req.name.clone(),
        };

        db.last_id += 1;
        db.items.push(new_item.clone());

        save_db(&db).await.map_err(|_| InternalError)?;

        Ok(Response::Created(Json(new_item)))
    }

    #[oai(path = "/items/:id", method = "get", tag = "Group::Items")]
    async fn get_item(
        &self,
        db: Data<&Db>,
        id: Path<u32>,
    ) -> Result<find_item::Response, find_item::Error> {
        use find_item::{Error::NotFound, Response};

        let db = db.lock().await;

        db.items
            .iter()
            .find(|item| item.id == *id)
            .cloned()
            .map(|item| Response::Ok(Json(item)))
            .ok_or_else(|| NotFound)
    }

    #[oai(path = "/items/:id", method = "put", tag = "Group::Items")]
    async fn update_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
        req: Json<ItemRequest>,
    ) -> Result<modify_item::Response, modify_item::Error> {
        use modify_item::{
            Error::{InternalError, NotFound, ValidationErrors},
            Response,
        };

        if let Err(e) = req.0.validate() {
            return Err(ValidationErrors(Json(e.into())));
        }

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
            save_db(&db).await.map_err(|_| InternalError)?;
            Ok(Response::Ok(Json(item)))
        } else {
            Err(NotFound)
        }
    }

    #[oai(path = "/items/:id", method = "delete", tag = "Group::Items")]
    async fn delete_item(
        &self,
        _auth: Authenticate,
        db: Data<&Db>,
        id: Path<u32>,
    ) -> Result<remove_item::Response, remove_item::Error> {
        use remove_item::{
            Error::{InternalError, NotFound},
            Response,
        };

        let mut db = db.lock().await;

        if let Some(pos) = db.items.iter().position(|item| item.id == *id) {
            db.items.remove(pos);

            save_db(&db).await.map_err(|_| InternalError)?;

            return Ok(Response::Ok(Json(DeletedMessage::new())));
        }

        Err(NotFound)
    }
}
