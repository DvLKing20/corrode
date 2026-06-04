use gtk4::{Picture, prelude::GtkWindowExt, gio};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub struct Wallpaper {
    wallpaper: Picture,
    window: Option<gtk4::ApplicationWindow>,
}

impl Wallpaper {
    pub fn new() -> Self {
        let wallpaper = Picture::new();
        let window = None;
        Self { window, wallpaper }
    }

    pub fn update(&self, texture: &gtk4::gdk::Texture) {
        self.wallpaper.set_paintable(Some(texture));
    }

    pub fn init(&mut self, app: &gtk4::Application) {
        let window = gtk4::ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(Layer::Background);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Left, true);

        self.wallpaper.set_content_fit(gtk4::ContentFit::Cover);

        window.set_child(Some(&self.wallpaper));

        window
            .set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);

        window.set_exclusive_zone(-1);

        window.present();

        self.window = Some(window);
    }
}
