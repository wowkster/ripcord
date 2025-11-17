use gpui::*;
use gpui_component::list::{List, ListState, ListDelegate, ListItem, ListEvent, ListSeparatorItem};
use gpui_component::IndexPath;

struct DmsListDelegate {
    items: Vec<String>,
    selected_index: Option<IndexPath>,
}