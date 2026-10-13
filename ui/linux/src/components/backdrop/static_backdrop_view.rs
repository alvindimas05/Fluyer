use crate::shared::AppState;
use gtk::gdk;
use gtk::glib;
use gtk::graphene;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct StaticBackdropView {
        pub app_state: RefCell<Option<Rc<AppState>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StaticBackdropView {
        const NAME: &'static str = "FluyerStaticBackdropView";
        type Type = super::StaticBackdropView;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for StaticBackdropView {}

    impl WidgetImpl for StaticBackdropView {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let width = self.obj().width() as f32;
            let height = self.obj().height() as f32;
            if width <= 0.0 || height <= 0.0 {
                return;
            }

            let bounds = graphene::Rect::new(0.0, 0.0, width, height);

            if let Some(app) = self.app_state.borrow().as_ref() {
                let source = app.settings.backdrop_source();
                if let Some(tex) = app.covers.backdrop(&app.engine, &source, self.obj().upcast_ref()) {
                    // ponytail: GSK blur without mesh warp; radius 50 gives smooth glass wash.
                    snapshot.push_blur(50.0);
                    snapshot.append_texture(&tex, &bounds);
                    snapshot.pop();
                } else {
                    snapshot.append_color(&gdk::RGBA::new(0.08, 0.08, 0.1, 1.0), &bounds);
                }
            } else {
                snapshot.append_color(&gdk::RGBA::new(0.08, 0.08, 0.1, 1.0), &bounds);
            }

            // Dark scrim overlay for contrast with UI elements
            snapshot.append_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.55), &bounds);
        }
    }
}

glib::wrapper! {
    pub struct StaticBackdropView(ObjectSubclass<imp::StaticBackdropView>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl StaticBackdropView {
    pub fn new(app_state: Rc<AppState>) -> Self {
        let obj: Self = glib::Object::new();
        *obj.imp().app_state.borrow_mut() = Some(app_state.clone());
        obj.set_hexpand(true);
        obj.set_vexpand(true);
        *app_state.backdrop_redraw_trigger.borrow_mut() = Some(obj.clone().upcast());
        obj
    }
}
