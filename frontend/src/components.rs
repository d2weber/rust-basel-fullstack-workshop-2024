use crate::controllers::*;
use dioxus::prelude::*;
use model::PostShopItem;

pub struct ListChanged;

#[component]
pub fn ShoppingList(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let items_request = use_resource(move || async move {
        change_signal.read();
        get_items(&list_uuid.read()).await
    });

    match &*items_request.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div { class: "grid place-items-center min-h-500",
                ul {
                    class: "menu bg-base-200 w-200 rounded-box gap-1",
                    for i in list {
                        li {
                            key: "{i.uuid}",
                            ShoppingListItemComponent{
                                display_name: i.title.clone(),
                                posted_by: i.posted_by.clone(),
                                list_id: list_uuid,
                                item_id: i.uuid.clone(),
                                change_signal
                            },
                        }
                    }
                }
            }
        },
        Some(Err(err)) => {
            rsx! {
                p {
                    "Error: {err}"
                }
            }
        }
        None => {
            rsx! {
                p {
                    "Loading items..."
                }
            }
        }
    }
}

#[component]
fn ShoppingListItemComponent(
    display_name: String,
    posted_by: String,
    list_id: String,
    item_id: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span {
                "posted by {posted_by}"
            }
            ItemDeleteButton {list_id, item_id, change_signal}
        }
    }
}

#[component]
pub fn ItemInput(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let mut item = use_signal(|| String::new());
    let mut author = use_signal(|| String::new());

    let onsubmit = move |_| {
        spawn({
            async move {
                let item_name = item.read().to_string();
                let author = author.read().to_string();
                if let Ok(_) = post_item(
                    &list_uuid.read(),
                    PostShopItem {
                        title: item_name,
                        posted_by: author,
                    },
                )
                .await
                {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}

#[component]
fn ItemDeleteButton(
    list_id: String,
    item_id: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    let onclick = move |_| {
        spawn({
            let list_id = list_id.clone();
            let item_id = item_id.clone();
            async move {
                if let Ok(_) = delete_item(&list_id, &item_id).await {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
    button {
        onclick: onclick,
        class: "btn btn-circle",
            svg {
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
                path {
                    d: "M6 18L18 6M6 6l12 12"
                }
            }
        }
    }
}
