use model::{CreateListResponse, PostShopItem, ShoppingListItem};

pub async fn get_items(list_id: &str) -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    reqwest::get(format!("http://localhost:3001/list/{list_id}/items"))
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await
}

pub async fn post_item(
    list_id: &str,
    item: PostShopItem,
) -> Result<ShoppingListItem, reqwest::Error> {
    reqwest::Client::new()
        .post(format!("http://localhost:3001/list/{list_id}/items"))
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await
}

pub async fn delete_item(list_id: &str, item_id: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!(
            "http://localhost:3001/list/{list_id}/items/{item_id}",
        ))
        .send()
        .await
        .map(|_| ())
}

async fn create_list() -> Result<CreateListResponse, reqwest::Error> {
    reqwest::Client::new()
        .get("http://localhost:3001/list")
        .send()
        .await?
        .json::<CreateListResponse>()
        .await
}
