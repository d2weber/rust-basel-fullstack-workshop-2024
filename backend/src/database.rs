use std::collections::HashMap;

#[derive(Clone)]
pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

impl ShoppingItem {
    pub fn to_model(&self, uuid: impl AsRef<str>) -> model::ShoppingListItem {
        model::ShoppingListItem {
            title: self.title.clone(),
            posted_by: self.creator.clone(),
            uuid: uuid.as_ref().to_owned(),
        }
    }
}

impl From<model::PostShopItem> for ShoppingItem {
    fn from(model::PostShopItem { title, posted_by }: model::PostShopItem) -> Self {
        Self {
            title,
            creator: posted_by,
        }
    }
}

type Uuid = String;

pub struct InMemoryDatabase {
    inner: HashMap<String, ShoppingItem>,
}
impl InMemoryDatabase {
    fn get_items(&self, uuid: &str) -> Option<&ShoppingItem> {
        self.inner.get(uuid)
    }

    pub fn insert_item(&mut self, uuid: impl AsRef<str>, item: ShoppingItem) {
        self.inner.insert(uuid.as_ref().to_owned(), item);
    }

    fn delete_item(&mut self, uuid: &str) {
        self.inner.remove(uuid);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &ShoppingItem)> {
        self.inner.iter()
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self {
            inner: [
                (
                    "asdf".to_owned(),
                    ShoppingItem {
                        title: "Pizza".to_owned(),
                        creator: "Douglas".to_string(),
                    },
                ),
                (
                    "asdf2".to_owned(),
                    ShoppingItem {
                        title: "10x Brocollo".to_owned(),
                        creator: "Alice".to_owned(),
                    },
                ),
            ]
            .into(),
        }
    }
}
