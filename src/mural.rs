use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

const FIREBASE_URL: &str = "https://mural-69f43-default-rtdb.firebaseio.com/mural.json";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MuralKind {
    Note,
    Photo,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MuralItem {
    pub id: u64,
    pub kind: MuralKind,
    pub title: String,
    pub text: String,
    pub image_url: String,
    pub x: f64,        // percentage 5.0 - 82.0
    pub y: f64,        // percentage 5.0 - 82.0
    pub rotation: f64, // degrees -6.0 to 6.0
    pub color_theme: String,
    pub pin_color: String,
}

fn get_default_items() -> Vec<MuralItem> {
    vec![
        MuralItem {
            id: 1,
            kind: MuralKind::Note,
            title: "Live Field Bulletin".to_string(),
            text: "Welcome to the globally shared Field Mural! 📌 Any note, photo, or dispatch you pin here is saved to the cloud and visible to all future visitors in real time.".to_string(),
            image_url: String::new(),
            x: 8.0,
            y: 8.0,
            rotation: -2.5,
            color_theme: "yellow".to_string(),
            pin_color: "red".to_string(),
        },
        MuralItem {
            id: 2,
            kind: MuralKind::Photo,
            title: "Expedition Basecamp".to_string(),
            text: "High-altitude atmospheric observatory & field station.".to_string(),
            image_url: "https://images.unsplash.com/photo-1464822759023-fed622ff2c3b?w=600&auto=format&fit=crop&q=80".to_string(),
            x: 38.0,
            y: 6.0,
            rotation: 2.0,
            color_theme: "parchment".to_string(),
            pin_color: "brass".to_string(),
        },
        MuralItem {
            id: 3,
            kind: MuralKind::Note,
            title: "Research Log".to_string(),
            text: "“In the middle of difficulty lies opportunity.” — Testing WebAssembly linear memory compilation.".to_string(),
            image_url: String::new(),
            x: 70.0,
            y: 12.0,
            rotation: -1.8,
            color_theme: "sage".to_string(),
            pin_color: "brass".to_string(),
        },
        MuralItem {
            id: 4,
            kind: MuralKind::Photo,
            title: "Botanical Specimen".to_string(),
            text: "Pinus sylvestris needle architecture.".to_string(),
            image_url: "https://images.unsplash.com/photo-1448375240586-882707db888b?w=600&auto=format&fit=crop&q=80".to_string(),
            x: 18.0,
            y: 52.0,
            rotation: 3.5,
            color_theme: "parchment".to_string(),
            pin_color: "green".to_string(),
        },
        MuralItem {
            id: 5,
            kind: MuralKind::Note,
            title: "Quote for Explorers".to_string(),
            text: "“Simplicity is prerequisite for reliability.” — Edsger W. Dijkstra".to_string(),
            image_url: String::new(),
            x: 55.0,
            y: 50.0,
            rotation: -3.0,
            color_theme: "rose".to_string(),
            pin_color: "red".to_string(),
        },
    ]
}

// Fetch live items from Firebase cloud
async fn api_fetch_items() -> Option<Vec<MuralItem>> {
    if let Ok(resp) = Request::get(FIREBASE_URL).send().await {
        if resp.ok() {
            if let Ok(maybe_items) = resp.json::<Option<Vec<Option<MuralItem>>>>().await {
                if let Some(items) = maybe_items {
                    let clean = items.into_iter().flatten().collect::<Vec<_>>();
                    if !clean.is_empty() {
                        return Some(clean);
                    }
                }
            }
        }
    }
    None
}

// Save items to Firebase cloud
async fn api_save_items(items: &[MuralItem]) {
    let _ = Request::put(FIREBASE_URL)
        .header("Content-Type", "application/json")
        .json(&items)
        .expect("serialize items")
        .send()
        .await;
}

#[derive(Clone, PartialEq)]
struct DragState {
    id: u64,
    start_mouse_x: f64,
    start_mouse_y: f64,
    start_item_x: f64,
    start_item_y: f64,
}

#[function_component(MuralPage)]
pub fn mural_page() -> Html {
    let items = use_state(get_default_items);
    let is_syncing = use_state(|| false);
    let drag_state = use_state(|| None::<DragState>);
    let board_ref = use_node_ref();
    let editing_id = use_state(|| None::<u64>);

    // Modal state for adding new item
    let show_add_modal = use_state(|| false);
    let new_kind = use_state(|| MuralKind::Note);
    let new_title = use_state(String::new);
    let new_text = use_state(String::new);
    let new_image_url = use_state(String::new);
    let new_color = use_state(|| "yellow".to_string());

    // 1. Initial Load: Fetch shared live board from Firebase
    {
        let items = items.clone();
        let is_syncing = is_syncing.clone();
        use_effect_with((), move |_| {
            let items = items.clone();
            let is_syncing = is_syncing.clone();
            is_syncing.set(true);
            spawn_local(async move {
                if let Some(live_items) = api_fetch_items().await {
                    items.set(live_items);
                } else {
                    // Seed Firebase with defaults if empty
                    let defaults = get_default_items();
                    api_save_items(&defaults).await;
                    items.set(defaults);
                }
                is_syncing.set(false);
            });
            || ()
        });
    }

    // Helper to sync changes to Firebase
    let sync_to_cloud = {
        let is_syncing = is_syncing.clone();
        Callback::from(move |items_to_save: Vec<MuralItem>| {
            let is_syncing = is_syncing.clone();
            is_syncing.set(true);
            spawn_local(async move {
                api_save_items(&items_to_save).await;
                is_syncing.set(false);
            });
        })
    };

    // Manual Refresh from Cloud
    let refresh_from_cloud = {
        let items = items.clone();
        let is_syncing = is_syncing.clone();
        Callback::from(move |_| {
            let items = items.clone();
            let is_syncing = is_syncing.clone();
            is_syncing.set(true);
            spawn_local(async move {
                if let Some(live_items) = api_fetch_items().await {
                    items.set(live_items);
                }
                is_syncing.set(false);
            });
        })
    };

    // Drag handlers
    let on_mouse_down = {
        let drag_state = drag_state.clone();
        Callback::from(move |(id, item_x, item_y, e): (u64, f64, f64, MouseEvent)| {
            drag_state.set(Some(DragState {
                id,
                start_mouse_x: e.client_x() as f64,
                start_mouse_y: e.client_y() as f64,
                start_item_x: item_x,
                start_item_y: item_y,
            }));
        })
    };

    let on_mouse_move = {
        let drag_state = drag_state.clone();
        let items = items.clone();
        let board_ref = board_ref.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(drag) = (*drag_state).clone() {
                if let Some(board) = board_ref.cast::<web_sys::Element>() {
                    let rect = board.get_bounding_client_rect();
                    let width = rect.width();
                    let height = rect.height();

                    if width > 0.0 && height > 0.0 {
                        let delta_x = ((e.client_x() as f64 - drag.start_mouse_x) / width) * 100.0;
                        let delta_y = ((e.client_y() as f64 - drag.start_mouse_y) / height) * 100.0;

                        let new_x = (drag.start_item_x + delta_x).clamp(2.0, 82.0);
                        let new_y = (drag.start_item_y + delta_y).clamp(2.0, 82.0);

                        let updated = (*items)
                            .iter()
                            .map(|item| {
                                if item.id == drag.id {
                                    let mut clone = item.clone();
                                    clone.x = (new_x * 10.0).round() / 10.0;
                                    clone.y = (new_y * 10.0).round() / 10.0;
                                    clone
                                } else {
                                    item.clone()
                                }
                            })
                            .collect::<Vec<_>>();

                        items.set(updated);
                    }
                }
            }
        })
    };

    // When drag ends, save coordinates to Firebase
    let on_mouse_up = {
        let drag_state = drag_state.clone();
        let items = items.clone();
        let sync_to_cloud = sync_to_cloud.clone();

        Callback::from(move |_| {
            if drag_state.is_some() {
                drag_state.set(None);
                sync_to_cloud.emit((*items).clone());
            }
        })
    };

    // Actions
    let delete_item = {
        let items = items.clone();
        let sync_to_cloud = sync_to_cloud.clone();
        Callback::from(move |id: u64| {
            let filtered = (*items)
                .iter()
                .filter(|item| item.id != id)
                .cloned()
                .collect::<Vec<_>>();
            items.set(filtered.clone());
            sync_to_cloud.emit(filtered);
        })
    };

    let reset_board = {
        let items = items.clone();
        let sync_to_cloud = sync_to_cloud.clone();
        Callback::from(move |_| {
            let defaults = get_default_items();
            items.set(defaults.clone());
            sync_to_cloud.emit(defaults);
        })
    };

    let open_add_modal = {
        let show_add_modal = show_add_modal.clone();
        let new_title = new_title.clone();
        let new_text = new_text.clone();
        let new_image_url = new_image_url.clone();
        Callback::from(move |_kind: MuralKind| {
            new_title.set(String::new());
            new_text.set(String::new());
            new_image_url.set(String::new());
            show_add_modal.set(true);
        })
    };

    let close_add_modal = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| {
            show_add_modal.set(false);
        })
    };

    let add_new_item = {
        let items = items.clone();
        let show_add_modal = show_add_modal.clone();
        let new_kind = new_kind.clone();
        let new_title = new_title.clone();
        let new_text = new_text.clone();
        let new_image_url = new_image_url.clone();
        let new_color = new_color.clone();
        let sync_to_cloud = sync_to_cloud.clone();

        Callback::from(move |_| {
            let next_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
            let title = if (*new_title).is_empty() {
                "Pinned Note".to_string()
            } else {
                (*new_title).clone()
            };

            let item = MuralItem {
                id: next_id,
                kind: (*new_kind).clone(),
                title,
                text: (*new_text).clone(),
                image_url: (*new_image_url).clone(),
                x: 35.0,
                y: 35.0,
                rotation: ((next_id % 7) as f64 - 3.0) * 1.5,
                color_theme: (*new_color).clone(),
                pin_color: if next_id % 2 == 0 { "brass".to_string() } else { "red".to_string() },
            };

            let mut current = (*items).clone();
            current.push(item);
            items.set(current.clone());
            sync_to_cloud.emit(current);
            show_add_modal.set(false);
        })
    };

    // Update text inline
    let update_item_text = {
        let items = items.clone();
        let sync_to_cloud = sync_to_cloud.clone();
        Callback::from(move |(id, new_content): (u64, String)| {
            let updated = (*items)
                .iter()
                .map(|item| {
                    if item.id == id {
                        let mut clone = item.clone();
                        clone.text = new_content.clone();
                        clone
                    } else {
                        item.clone()
                    }
                })
                .collect::<Vec<_>>();
            items.set(updated.clone());
            sync_to_cloud.emit(updated);
        })
    };

    html! {
        <div class="page-container mural-outer-container">
            <header class="section-header">
                <span class="section-kicker">{ "COMMUNITY BULLETIN · REAL-TIME CLOUD" }</span>
                <h2>{ "The Global Field Station Cork Board" }</h2>
                <p>{ "A collaborative bulletin board shared live across all visitors. Pin notes, post images from any website, and leave dispatches for future explorers." }</p>
            </header>

            // Toolbar
            <div class="corkboard-toolbar">
                <div class="toolbar-left">
                    <button class="btn btn-primary btn-sm" onclick={
                        let open_add_modal = open_add_modal.clone();
                        let new_kind = new_kind.clone();
                        Callback::from(move |_| {
                            new_kind.set(MuralKind::Note);
                            open_add_modal.emit(MuralKind::Note);
                        })
                    }>
                        { "📌 Pin a Note" }
                    </button>
                    <button class="btn btn-secondary btn-sm" onclick={
                        let open_add_modal = open_add_modal.clone();
                        let new_kind = new_kind.clone();
                        Callback::from(move |_| {
                            new_kind.set(MuralKind::Photo);
                            open_add_modal.emit(MuralKind::Photo);
                        })
                    }>
                        { "🖼️ Pin an Image" }
                    </button>
                    <button class="btn btn-subtle btn-sm" onclick={refresh_from_cloud} title="Fetch latest notes posted by other visitors">
                        { "🔄 Refresh Board" }
                    </button>
                </div>
                <div class="toolbar-right">
                    {
                        if *is_syncing {
                            html! { <span class="corkboard-sync-badge syncing">{ "🔄 Syncing with Cloud..." }</span> }
                        } else {
                            html! { <span class="corkboard-sync-badge synced">{ "🟢 Live Shared (Firebase)" }</span> }
                        }
                    }
                    <button class="btn btn-subtle btn-sm" onclick={reset_board} title="Reset to initial welcome dispatches">
                        { "↺ Reset Board" }
                    </button>
                </div>
            </div>

            // The Cork Board
            <div
                ref={board_ref}
                class="cork-board-canvas"
                onmousemove={on_mouse_move}
                onmouseup={on_mouse_up}
                onmouseleave={{
                    let drag_state = drag_state.clone();
                    Callback::from(move |_| drag_state.set(None))
                }}
            >
                {
                    items.iter().map(|item| {
                        let item_id = item.id;
                        let item_x = item.x;
                        let item_y = item.y;
                        let on_mouse_down = on_mouse_down.clone();
                        let delete_item = delete_item.clone();
                        let update_item_text = update_item_text.clone();
                        let editing_id = editing_id.clone();
                        let is_editing = *editing_id == Some(item.id);

                        let card_style = format!(
                            "left: {}%; top: {}%; transform: rotate({}deg);",
                            item.x, item.y, item.rotation
                        );

                        let theme_class = format!("mural-pin-card theme-{} kind-{:?}", item.color_theme, item.kind).to_lowercase();
                        let pin_class = format!("pushpin pin-{}", item.pin_color);

                        html! {
                            <div
                                key={item.id}
                                class={classes!(theme_class, is_editing.then_some("is-editing"))}
                                style={card_style}
                                onmousedown={Callback::from(move |e: MouseEvent| {
                                    on_mouse_down.emit((item_id, item_x, item_y, e));
                                })}
                            >
                                // Pushpin at top
                                <div class={pin_class}></div>

                                // Action buttons (Delete & Edit)
                                <div class="pin-card-actions">
                                    <button
                                        class="pin-action-btn edit-btn"
                                        title="Edit text"
                                        onclick={{
                                            let editing_id = editing_id.clone();
                                            Callback::from(move |e: MouseEvent| {
                                                e.stop_propagation();
                                                if is_editing {
                                                    editing_id.set(None);
                                                } else {
                                                    editing_id.set(Some(item_id));
                                                }
                                            })
                                        }}
                                    >
                                        { if is_editing { "✓" } else { "✎" } }
                                    </button>
                                    <button
                                        class="pin-action-btn delete-btn"
                                        title="Unpin from board"
                                        onclick={{
                                            let delete_item = delete_item.clone();
                                            Callback::from(move |e: MouseEvent| {
                                                e.stop_propagation();
                                                delete_item.emit(item_id);
                                            })
                                        }}
                                    >
                                        { "✕" }
                                    </button>
                                </div>

                                // Card Content
                                {
                                    match item.kind {
                                        MuralKind::Photo => html! {
                                            <div class="polaroid-wrapper">
                                                <div class="polaroid-photo">
                                                    <img src={item.image_url.clone()} alt={item.title.clone()} draggable="false" />
                                                </div>
                                                <div class="polaroid-caption">
                                                    <h5>{ &item.title }</h5>
                                                    {
                                                        if is_editing {
                                                            let update_item_text = update_item_text.clone();
                                                            html! {
                                                                <textarea
                                                                    class="pin-inline-editor"
                                                                    value={item.text.clone()}
                                                                    oninput={Callback::from(move |e: InputEvent| {
                                                                        let target: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                                                        update_item_text.emit((item_id, target.value()));
                                                                    })}
                                                                    onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                                                                />
                                                            }
                                                        } else {
                                                            html! { <p class="pin-text">{ &item.text }</p> }
                                                        }
                                                    }
                                                </div>
                                            </div>
                                        },
                                        MuralKind::Note => html! {
                                            <div class="sticky-note-content">
                                                <h4 class="note-title">{ &item.title }</h4>
                                                {
                                                    if is_editing {
                                                        let update_item_text = update_item_text.clone();
                                                        html! {
                                                            <textarea
                                                                class="pin-inline-editor note-editor"
                                                                value={item.text.clone()}
                                                                oninput={Callback::from(move |e: InputEvent| {
                                                                    let target: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                                                    update_item_text.emit((item_id, target.value()));
                                                                })}
                                                                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                                                            />
                                                        }
                                                    } else {
                                                        html! { <p class="pin-text note-body">{ &item.text }</p> }
                                                    }
                                                }
                                            </div>
                                        }
                                    }
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Add Item Modal
            {
                if *show_add_modal {
                    let is_photo = *new_kind == MuralKind::Photo;
                    html! {
                        <div class="lightbox-overlay" onclick={close_add_modal.clone()}>
                            <div class="mural-modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                <button class="lightbox-close-btn" onclick={close_add_modal}>{ "✕" }</button>
                                <h3>{ if is_photo { "🖼️ Pin a New Image" } else { "📌 Pin a New Note" } }</h3>
                                <p class="mural-modal-sub">{ "Add your thoughts, quotes, or image URLs from any website to the global live board." }</p>

                                <div class="mural-form-group">
                                    <label>{ "Title or Caption:" }</label>
                                    <input
                                        type="text"
                                        class="mural-input"
                                        placeholder="e.g. Field Observation, Mountain Sunset..."
                                        value={(*new_title).clone()}
                                        oninput={{
                                            let new_title = new_title.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                new_title.set(target.value());
                                            })
                                        }}
                                    />
                                </div>

                                {
                                    if is_photo {
                                        let new_image_url = new_image_url.clone();
                                        html! {
                                            <div class="mural-form-group">
                                                <label>{ "Image URL (from any website or Unsplash):" }</label>
                                                <input
                                                    type="url"
                                                    class="mural-input"
                                                    placeholder="https://images.unsplash.com/photo-..."
                                                    value={(*new_image_url).clone()}
                                                    oninput={Callback::from(move |e: InputEvent| {
                                                        let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        new_image_url.set(target.value());
                                                    })}
                                                />
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                                <div class="mural-form-group">
                                    <label>{ if is_photo { "Description or Quote:" } else { "Note Content:" } }</label>
                                    <textarea
                                        class="mural-textarea"
                                        rows="4"
                                        placeholder="Write your observation, notes, or quote here..."
                                        value={(*new_text).clone()}
                                        oninput={{
                                            let new_text = new_text.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let target: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                                new_text.set(target.value());
                                            })
                                        }}
                                    />
                                </div>

                                <div class="mural-form-group">
                                    <label>{ "Paper Theme:" }</label>
                                    <div class="color-picker-row">
                                        {
                                            ["yellow", "parchment", "sage", "rose"].iter().map(|color| {
                                                let color_str = color.to_string();
                                                let new_color = new_color.clone();
                                                let is_selected = *new_color == color_str;
                                                html! {
                                                    <button
                                                        type="button"
                                                        class={classes!("color-btn", format!("color-{}", color), is_selected.then_some("selected"))}
                                                        onclick={Callback::from(move |_| new_color.set(color_str.clone()))}
                                                    >
                                                        { *color }
                                                    </button>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>

                                <div class="mural-modal-actions">
                                    <button class="btn btn-primary" onclick={add_new_item}>{ "📌 Pin to Global Board" }</button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
