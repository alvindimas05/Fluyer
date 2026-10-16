use crate::shared::state::LibraryState;
use gtk::gio;
use gtk::prelude::*;

const TRACK_SORTS: [&str; 4] = ["Album", "Title", "Artist", "Duration"];
const ALBUM_SORTS: [&str; 4] = ["Name", "Artist", "Year", "Track Count"];

/// Toolbar sort control (macOS `SortMenu`). Keys follow the current mode; the
/// direction is shared. Radio items backed by stateful `gio` actions.
pub fn sort_menu(library: &LibraryState) -> gtk::MenuButton {
    let group = gio::SimpleActionGroup::new();
    group.add_action(&radio("track-sort", library.track_sort(), {
        let l = library.clone();
        move |v| l.set_track_sort(v)
    }));
    group.add_action(&radio("album-sort", library.album_sort(), {
        let l = library.clone();
        move |v| l.set_album_sort(v)
    }));
    let order = gio::SimpleAction::new_stateful(
        "ascending",
        Some(&bool::static_variant_type()),
        &library.sort_ascending().to_variant(),
    );
    {
        let l = library.clone();
        order.connect_change_state(move |a, v| {
            if let Some(v) = v.and_then(|v| v.get::<bool>()) {
                a.set_state(&v.to_variant());
                l.set_sort_ascending(v);
            }
        });
    }
    group.add_action(&order);

    let button = gtk::MenuButton::new();
    button.set_icon_name("view-sort-ascending-symbolic");
    button.set_tooltip_text(Some("Sort"));
    button.insert_action_group("sort", Some(&group));
    button.set_menu_model(Some(&model(library.mode())));

    // Rebuild on mode change so the keys match Songs vs Albums.
    let b = button.clone();
    library.connect_mode_notify(move |l| b.set_menu_model(Some(&model(l.mode()))));
    button
}

fn radio(name: &str, initial: u32, set: impl Fn(u32) + 'static) -> gio::SimpleAction {
    let a = gio::SimpleAction::new_stateful(name, Some(&u32::static_variant_type()), &initial.to_variant());
    a.connect_change_state(move |a, v| {
        if let Some(v) = v.and_then(|v| v.get::<u32>()) {
            a.set_state(&v.to_variant());
            set(v);
        }
    });
    a
}

fn model(mode: u32) -> gio::Menu {
    let (action, keys) = if mode == 1 {
        ("sort.album-sort", ALBUM_SORTS)
    } else {
        ("sort.track-sort", TRACK_SORTS)
    };
    let by = gio::Menu::new();
    for (i, key) in keys.iter().enumerate() {
        let item = gio::MenuItem::new(Some(key), None);
        item.set_action_and_target_value(Some(action), Some(&(i as u32).to_variant()));
        by.append_item(&item);
    }
    let order = gio::Menu::new();
    for (label, asc) in [("Ascending", true), ("Descending", false)] {
        let item = gio::MenuItem::new(Some(label), None);
        item.set_action_and_target_value(Some("sort.ascending"), Some(&asc.to_variant()));
        order.append_item(&item);
    }
    let menu = gio::Menu::new();
    menu.append_section(Some("Sort By"), &by);
    menu.append_section(Some("Order"), &order);
    menu
}
