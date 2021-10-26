<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> done messages get route
use crate::AppStorage;
use crate::types::{
    dtos::MessageDto,
    body::SuccessBody,
    responses::MessageResponse,
<<<<<<< HEAD
};

use bee_message::{Message, MessageId};
use bee_storage::access::Fetch;

use axum::{
    extract::{Path, Extension},
    handler::{get, post},
    response::Json,
    Router,
    routing::BoxRoute,
};
use serde_json::{Value, json};
use uuid::Uuid;

use std::{
    sync::Arc,
    str::FromStr,
};

pub fn api_routes() -> Router<BoxRoute> {
    
=======
use bee_storage_sled::{
    storage::Storage,
=======
>>>>>>> done messages get route
};

use bee_message::{Message, MessageId};
use bee_storage::access::Fetch;

use axum::{
    extract::{Path, Extension},
    handler::{get, post},
    response::Json,
    Router,
    routing::BoxRoute,
};
use serde_json::{Value, json};
use uuid::Uuid;

<<<<<<< HEAD
pub fn api_routes(storage: &Storage) -> Router<BoxRoute> {
>>>>>>> Updated parents
=======
use std::{
    sync::Arc,
    str::FromStr,
};

pub fn api_routes() -> Router<BoxRoute> {
    
>>>>>>> done messages get route
    Router::new()
        .route("/", get(get_handler))
        .route("/", post(post_handler))
        .route("/:messageId", get(get_id_handler))
        .route("/:messageId/metadata", get(get_id_metadata_handler))
        .route("/:messageId/raw", get(get_id_raw_handler))
        .route("/:messageId/children", get(get_id_children_handler))
        .boxed()
}

<<<<<<< HEAD
<<<<<<< HEAD
pub async fn get_handler(Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {

    Json(json!({ "test": 11 }))
}

pub async fn post_handler(Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

async fn get_id_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {

    match Fetch::<MessageId, Message>::fetch(&*(
        app_storage.storage.lock().unwrap()),
            match &MessageId::from_str(&messageId.to_string()) {
                Ok(message_id) => message_id,
                Err(e) => return Json(json!({ "error": "could not parse message ID" })),
            }
        ) {
            Ok(message) => match message {
                Some(message) => Json(match serde_json::to_value(&SuccessBody::new(MessageResponse(MessageDto::from(&message)))) {
                    Ok(data) => data,
                    Err(e) => json!({ "error": "messagedto conversion failed" }),
                }),
                None => Json(json!({ "error": "could not find message" })),
                }
            Err(e) => Json(json!({ "error": "could not get message from storage" })),
    }
}

pub async fn get_id_metadata_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

pub async fn get_id_raw_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

pub async fn get_id_children_handler(Path(messageId): Path<Uuid>) -> Json<Value> {
    Json(json!({ "test": 11 }))
=======
pub async fn get_handler() -> Html<&'static str> {
    Html("<h1>You are in /messages with post methode</h1>")
=======
pub async fn get_handler(Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {

    Json(json!({ "test": 11 }))
>>>>>>> done messages get route
}

pub async fn post_handler(Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

async fn get_id_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {

    match Fetch::<MessageId, Message>::fetch(&*(
        app_storage.storage.lock().unwrap()),
            match &MessageId::from_str(&messageId.to_string()) {
                Ok(message_id) => message_id,
                Err(e) => return Json(json!({ "error": "could not parse message ID" })),
            }
        ) {
            Ok(message) => match message {
                Some(message) => Json(match serde_json::to_value(&SuccessBody::new(MessageResponse(MessageDto::from(&message)))) {
                    Ok(data) => data,
                    Err(e) => json!({ "error": "messagedto conversion failed" }),
                }),
                None => Json(json!({ "error": "could not find message" })),
                }
            Err(e) => Json(json!({ "error": "could not get message from storage" })),
    }
}

pub async fn get_id_metadata_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

pub async fn get_id_raw_handler(Path(messageId): Path<Uuid>, Extension(app_storage): Extension<Arc<AppStorage>>) -> Json<Value> {
    Json(json!({ "test": 11 }))
}

<<<<<<< HEAD
pub async fn get_id_children_handler(Path(messageId): Path<Uuid>) -> Html<&'static str> {
    Html("<h1>You are in /messages/{}/children</h1>")
>>>>>>> Updated parents
=======
pub async fn get_id_children_handler(Path(messageId): Path<Uuid>) -> Json<Value> {
    Json(json!({ "test": 11 }))
>>>>>>> done messages get route
}