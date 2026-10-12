use crate::screens::home::albums::AlbumGridView;
use crate::screens::home::music_grid::MusicGridView;
use crate::screens::home::player_bar::PlayerBarView;
use crate::screens::home::queue::QueueView;
use crate::screens::home::toolbar::ToolbarView;
use crate::shared::AppState;
use gtk::prelude::*;
use std::rc::Rc;

pub struct HomeView {
    pub split_view: adw::OverlaySplitView,
}

impl HomeView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let split_view = adw::OverlaySplitView::new();
        split_view.set_sidebar_position(gtk::PackType::End);
        split_view.set_show_sidebar(false);
        split_view.set_min_sidebar_width(300.0);
        split_view.set_max_sidebar_width(380.0);

        // Sidebar: Queue panel
        let queue_view = QueueView::new(app_state.clone());
        split_view.set_sidebar(Some(&queue_view.container));

        // Connect queue is_open notify
        {
            let sv = split_view.clone();
            app_state.queue.connect_is_open_notify(move |qs| {
                sv.set_show_sidebar(qs.is_open());
            });
        }

        // Content
        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let toolbar = ToolbarView::new(app_state.clone());
        content_box.append(&toolbar.header_bar);

        let library_stack = gtk::Stack::new();
        library_stack.set_vexpand(true);
        library_stack.set_hexpand(true);
        library_stack.set_transition_type(gtk::StackTransitionType::Crossfade);

        let music_grid = MusicGridView::new(app_state.clone());
        let album_grid = AlbumGridView::new(app_state.clone());

        library_stack.add_named(&music_grid.container, Some("songs"));
        library_stack.add_named(&album_grid.container, Some("albums"));

        {
            let stack = library_stack.clone();
            app_state.library.connect_mode_notify(move |ls| {
                if ls.mode() == 0 {
                    stack.set_visible_child_name("songs");
                } else {
                    stack.set_visible_child_name("albums");
                }
            });
        }

        content_box.append(&library_stack);

        let player_bar = PlayerBarView::new(app_state.clone());
        content_box.append(&player_bar.container);

        split_view.set_content(Some(&content_box));

        Self { split_view }
    }
}
