pub mod messages;

<<<<<<< HEAD
<<<<<<< HEAD
=======
use bee_storage_sled::storage::Storage;

>>>>>>> Updated parents
=======
>>>>>>> done messages get route
use axum::{
    handler::get,
    Router,
    response::Html,
    routing::BoxRoute
};

<<<<<<< HEAD
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
=======
pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        //.nest("/test", test::api_routes())
        .nest("messages", messages::api_routes())
>>>>>>> done messages get route
        .boxed()
}

async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /v1</h1>")
}