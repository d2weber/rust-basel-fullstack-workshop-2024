use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShoppingListItem {
    pub title: String,
    pub posted_by: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostShopItem {
    pub title: String,
    pub posted_by: String,
}
