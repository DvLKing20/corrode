use crate::wallpaper::background::Wallpaper;
use crate::wallpaper::gallery::Gallery;
use gtk4::Orientation::Vertical;
use gtk4::gdk::prelude::{DisplayExt, MonitorExt};
use gtk4::gio::prelude::ListModelExt;
use gtk4::glib::object::Cast;
use gtk4::prelude::{BoxExt, GtkWindowExt, WidgetExt};
use gtk4::{Picture, gdk, gio};
use gtk4_layer_shell::LayerShell;
use std::cell::Cell;

pub struct Picker {
    picture1: Picture,
    picture2: Picture,
    container: gtk4::Box,
    stack: gtk4::Stack,
    pub gallery: Gallery,
    window: gtk4::ApplicationWindow,
    is_using: Cell<bool>,
}

impl Picker {
    pub fn new(app: &gtk4::Application) -> Self {
        let picture1 = Picture::new();
        let picture2 = Picture::new();
        let is_using = Cell::new(true);
        let stack = gtk4::Stack::new();
        let container = gtk4::Box::new(Vertical, 0);
        container.append(&stack);
        let window = gtk4::ApplicationWindow::new(app);

        stack.add_child(&picture1);
        stack.add_child(&picture2);

        let gallery = Gallery::new();

        Self {
            picture1,
            picture2,
            container,
            stack,
            gallery,
            window,
            is_using,
       }
    }

    pub fn init(&self) {
        self.window.init_layer_shell();
        let (w, h) = self.cal_geo();
        self.window.set_default_size(w, h);
        self.window.set_child(Some(&self.container));
        self.window.set_layer(gtk4_layer_shell::Layer::Top);
        self.window.set_margin(gtk4_layer_shell::Edge::Bottom, 50);
        self.window
            .set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);
        self.picture1.set_content_fit(gtk4::ContentFit::Cover);
        self.picture2.set_content_fit(gtk4::ContentFit::Cover);
        self.stack.set_transition_duration(200);
        self.window.present();
    }

    pub unsafe fn update(
        &self,
        file: &gio::File,
        direction: gtk4::StackTransitionType,
        wallpaper: *const Wallpaper,
    ) {
        let Ok(texture) = gdk::Texture::from_file(file) else {
            return;
        };
        let target = if self.is_using.get() {
            &self.picture1
        } else {
            &self.picture2
        };
        self.stack.set_transition_type(direction);
        target.set_paintable(Some(&texture));
        (*wallpaper).update(&texture);
        self.stack.set_visible_child(target);
        self.is_using.set(!self.is_using.get());
    }

    fn cal_geo(&self) -> (i32, i32) {
        let default = (580, 240);
        let Some(display) = gdk::Display::default() else {
            return default;
        };

        let monitors = display.monitors();

        let Ok(monitor) = monitors
            .item(0)
            .expect("No monitors found")
            .downcast::<gdk::Monitor>()
        else {
            return default;
        };

        // Now you can get the geometry
        let geometry = monitor.geometry();

        let ui_w = (geometry.width() as f32 * 0.50) as i32;
        let ui_h = (geometry.height() as f32 * 0.50) as i32;
        (ui_w, ui_h)
    }

    pub unsafe fn setup_controls(picker: *const Picker, wallpaper: *const Wallpaper) {
        let controller = gtk4::EventControllerKey::new();

        controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Left => unsafe {
                    if let Some(file) = (*picker).gallery.prev() {
                        (*picker).update(file, gtk4::StackTransitionType::OverLeftRight, wallpaper);
                    }
                },

                gdk::Key::Right => unsafe {
                    if let Some(file) = (*picker).gallery.next() {
                        (*picker).update(file, gtk4::StackTransitionType::OverRightLeft, wallpaper);
                    }
                },

                gdk::Key::Escape => unsafe {
                    (*picker).window.destroy();
                    drop(Box::from_raw(picker as *mut Self));
                },
                _ => {}
            }
            gtk4::glib::Propagation::Proceed
        });

        (*picker).window.add_controller(controller);
    }
}
