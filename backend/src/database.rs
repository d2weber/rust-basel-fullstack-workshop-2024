use std::collections::HashMap;

pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

type UUID = String;

pub struct InMemoryDatabase {
    inner: HashMap<String, ShoppingItem>,
}
impl InMemoryDatabase {
    fn get_items(&self, uuid: &str) -> Option<&ShoppingItem> {
        self.inner.get(uuid)
    }

    fn insert_item(&mut self, uuid: &str, item: ShoppingItem) {
        self.inner.insert(uuid.to_string(), item);
    }

    fn delete_item(&mut self, uuid: &str) {
        self.inner.remove(uuid);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UUID, &ShoppingItem)> {
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
