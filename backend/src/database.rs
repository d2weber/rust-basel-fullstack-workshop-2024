use std::collections::HashMap;

#[derive(Clone)]
pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

struct ShoppingList {
    list: HashMap<String, ShoppingItem>,
}

impl ShoppingList {
    fn iter(&self) -> impl Iterator<Item = (&Uuid, &ShoppingItem)> {
        self.list.iter()
    }
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self {
            list: [
                (
                    "6855cfc9-78fd-4b66-8671-f3c90ac2abd8".to_string(),
                    ShoppingItem {
                        title: "Coffee".to_string(),
                        creator: "Roland".to_string(),
                    },
                ),
                (
                    "3d778d1c-5a4e-400f-885d-10212027382d".to_string(),
                    ShoppingItem {
                        title: "Tomato Seeds".to_string(),
                        creator: "Tania".to_string(),
                    },
                ),
            ]
            .into(),
        }
    }
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
    inner: HashMap<String, ShoppingList>,
}
impl InMemoryDatabase {
    pub fn insert_item(
        &mut self,
        list_uuid: impl AsRef<str>,
        item_uuid: impl AsRef<str>,
        shopping_item: ShoppingItem,
    ) {
        self.inner.get_mut(list_uuid.as_ref()).and_then(|list| {
            list.list
                .insert(item_uuid.as_ref().to_owned(), shopping_item)
        });
    }

    pub fn delete_item(&mut self, list_uuid: impl AsRef<str>, item_uuid: impl AsRef<str>) {
        self.inner
            .get_mut(list_uuid.as_ref())
            .and_then(|list| list.list.remove(item_uuid.as_ref()));
    }

    pub fn create_list(&mut self, list_uuid: impl AsRef<str>) {
        self.inner
            .insert(list_uuid.as_ref().to_owned(), ShoppingList::default());
    }

    pub fn iter_list(
        &self,
        list_uuid: impl AsRef<str>,
    ) -> Option<impl Iterator<Item = (&Uuid, &ShoppingItem)>> {
        self.inner.get(list_uuid.as_ref()).map(|l| l.iter())
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        Self {
            inner: [(
                "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string(),
                ShoppingList::default(),
            )]
            .into(),
        }
    }
}
