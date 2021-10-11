pub mod v1;

<<<<<<< HEAD
=======
use bee_storage_sled::storage::Storage;

>>>>>>> Updated parents
use axum::{
    handler::get,
    Router,
    response::Html,
    routing::BoxRoute
};

<<<<<<< HEAD
pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .nest("/v1", v1::api_routes())
=======
pub fn api_routes(storage: &Storage) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .nest("/v1", v1::api_routes(storage))
>>>>>>> Updated parents
        .boxed()
}

async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /api</h1>")
}