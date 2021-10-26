pub mod api;

<<<<<<< HEAD
<<<<<<< HEAD
=======
use bee_storage_sled::storage::Storage;

>>>>>>> Updated parents
=======
>>>>>>> done messages get route
use axum::{
    response::Html,
    handler::get,
    Router,
    routing::BoxRoute
};

<<<<<<< HEAD
<<<<<<< HEAD
pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .nest("/api", api::api_routes())
=======
pub fn api_routes(storage: &Storage) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .nest("/api", api::api_routes(storage))
>>>>>>> Updated parents
=======
pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .nest("/api", api::api_routes())
>>>>>>> done messages get route
        .boxed()
}


async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /</h1>")
}