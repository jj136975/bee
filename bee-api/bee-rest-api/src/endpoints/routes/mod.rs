pub mod api;

<<<<<<< HEAD
=======
use bee_storage_sled::storage::Storage;

>>>>>>> Updated parents
use axum::{
    response::Html,
    handler::get,
    Router,
    routing::BoxRoute
};

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
        .boxed()
}


async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /</h1>")
}