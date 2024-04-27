use std::sync::{Arc, RwLock};

use crate::database::InMemoryDatabase;
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use model::ShoppingListItem;

type Database = Arc<RwLock<InMemoryDatabase>>;

pub async fn get_items(State(state): State<Database>) -> impl IntoResponse {
    let items: Vec<ShoppingListItem> = state
        .read()
        .unwrap()
        .iter()
        .map(|(uuid, item)| ShoppingListItem {
            title: item.title.clone(),
            posted_by: item.creator.clone(),
            uuid: uuid.clone(),
        })
        .collect();
    Json(items)
}
