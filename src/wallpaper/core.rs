use gtk4::prelude::GtkWindowExt;
use gtk4_layer_shell::{LayerShell, Layer, Edge};

use crate::wallpaper::ui::UiConfig;
use crate::wallpaper::wallpaper::WallpaperConfig;

struct Core {
    window: gtk4::ApplicationWindow,
    wallpaper: WallpaperConfig,
    ui: UiConfig,
}

impl Core {
    pub fn new(app: &gtk4::Application) -> Self {
        let window = gtk4::ApplicationWindow::new(app);
        let wallpaper = WallpaperConfig::new();
        let ui = UiConfig::new();
        Self {
            window,
            wallpaper,
            ui
        }
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
