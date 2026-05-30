use gtk4::Orientation::Vertical;
use gtk4::gdk::prelude::{DisplayExt, MonitorExt};
use gtk4::gio::prelude::ListModelExt;
use gtk4::glib::object::Cast;
use gtk4::prelude::{BoxExt, GtkWindowExt, RootExt};
use gtk4::{Picture, gdk, gio};
use gtk4_layer_shell::LayerShell;

pub struct Picker {
    container: gtk4::Box,
    stack: gtk4::Stack,
    window: gtk4::ApplicationWindow,
}

impl Picker {
    pub fn new(app: &gtk4::Application) -> Self {
        let stack = gtk4::Stack::new();
        let container = gtk4::Box::new(Vertical, 0);
        container.append(&stack);
        let window = gtk4::ApplicationWindow::new(app);

        Self {
            container,
            stack,
            window,
        }
    }

    pub fn init(&self) {
        self.window.init_layer_shell();
        let (w, h) = self.cal_geo();
        self.window.set_default_size(w, h);
        self.window.set_child(Some(&self.container));
        self.window.set_layer(gtk4_layer_shell::Layer::Top);
        self.window.set_margin(gtk4_layer_shell::Edge::Bottom, 50);
        self.stack
            .set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
        self.stack.set_transition_duration(500);
        self.window.present();
    }

    pub fn update(&self, file: &gio::File) {
        while let Some(child) = self.stack.visible_child() {
            self.stack.remove(&child);
        }
        let wallpaper = Picture::for_file(file);
        self.stack.add_child(&wallpaper);
        self.stack.set_visible_child(&wallpaper);
    }

    fn cal_geo(&self) -> (i32, i32) {
        let default = (580, 240);
        let Some(display) = gdk::Display::default() else {
            return default;
        };

        let monitors = display.monitors();

        let monitor = monitors
            .item(0)
            .expect("No monitors found")
            .downcast::<gdk::Monitor>()
            .expect("Could not cast to GdkMonitor");

        // Now you can get the geometry
        let geometry = monitor.geometry();

        let ui_w = (geometry.width() as f32 * 0.28) as i32;
        let ui_h = (geometry.height() as f32 * 0.26) as i32;
        (ui_w, ui_h)
    }
}
