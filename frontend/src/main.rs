mod components;
mod controllers;

use components::*;
use dioxus::prelude::*;
use model::{PostShopItem, ShoppingListItem};

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let change_signal = use_signal(|| ListChanged);
    let rust_basel = "Rust Basel";
    rsx! {
        h1{
            "Welcome to {rust_basel}"
        }
        button{
            class: "btn",
            "My stylish button"
        }
        ShoppingList{change_signal}
        ItemInput{change_signal}
    }
}

async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = "http://localhost:3001/items";
    reqwest::get(url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await
}

async fn post_item(item: PostShopItem) -> Result<ShoppingListItem, reqwest::Error> {
    reqwest::Client::new()
        .post("http://localhost:3001/items")
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await
}
