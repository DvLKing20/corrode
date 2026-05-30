use crate::wallpaper::wallpaper::WallpaperConfig;
use gtk4::{Picture, prelude::GtkWindowExt, gio};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

struct Wallpaper {
    wallpaper: Picture,
    window: gtk4::ApplicationWindow,
}

impl Wallpaper {
    pub fn new(app: &gtk4::Application, wallpaper: &gio::File) -> Self {
        let window = gtk4::ApplicationWindow::new(app);
        Self { window, wallpaper }
    }

    pub fn init(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(Layer::Background);
        self.window.set_anchor(Edge::Top, true);
        self.window.set_anchor(Edge::Bottom, true);
        self.window.set_anchor(Edge::Right, true);
        self.window.set_anchor(Edge::Left, true);

        self.window
            .set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);

        self.window.set_exclusive_zone(-1);

        self.window.present();
    }
}
