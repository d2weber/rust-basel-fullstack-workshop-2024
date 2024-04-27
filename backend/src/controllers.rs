use std::sync::{Arc, RwLock};

use crate::database::{InMemoryDatabase, ShoppingItem};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use model::{PostShopItem, ShoppingListItem};
use uuid::Uuid;

type Database = Arc<RwLock<InMemoryDatabase>>;

pub async fn get_items(State(state): State<Database>) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state
        .read()
        .unwrap()
        .iter()
        .map(|(uuid, item)| item.to_model(uuid))
        .collect();
    Json(items)
}

pub async fn add_item(
    State(state): State<Database>,
    Json(post_request): Json<PostShopItem>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return (StatusCode::SERVICE_UNAVAILABLE).into_response();
    };

    let uuid = Uuid::new_v4().to_string();
    let item: ShoppingItem = post_request.into();
    db.insert_item(&uuid, item.clone());
    (
        StatusCode::OK,
        Json(ShoppingListItem {
            title: item.title,
            posted_by: item.creator,
            uuid,
        }),
    )
        .into_response()
}

pub async fn delete_item(
    State(state): State<Database>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let Ok(mut db) = state.write() else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };

    db.delete_item(uuid.to_string());

    StatusCode::OK
}
