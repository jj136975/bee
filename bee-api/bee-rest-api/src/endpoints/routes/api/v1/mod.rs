pub mod messages;

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
        //.nest("/test", test::api_routes())
        .nest("messages", messages::api_routes())
=======
pub fn api_routes(storage: &Storage) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        //.nest("/test", test::api_routes())
        .nest("messages", messages::api_routes(storage))
>>>>>>> Updated parents
        .boxed()
}

async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /v1</h1>")
}