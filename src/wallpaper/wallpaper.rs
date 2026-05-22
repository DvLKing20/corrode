use gtk4::{
    ApplicationWindow,Picture,
    gdk::{
        Texture,
        prelude::{DisplayExt, MonitorExt},
    },
    gio::prelude::ListModelExt,
    glib::object::Cast,
    prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub struct WallpaperConfig {
    window: ApplicationWindow,
    wallpaper: Picture,
}

impl WallpaperConfig {
    pub fn new(app: &gtk4::Application) -> Self {
        let window = ApplicationWindow::builder().application(app).build();

        let wallpaper = Picture::new();
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


        let bytes = Self::get_wallpaper();

        if let Ok(texture) = Texture::from_bytes(&bytes) {
           self.wallpaper.set_paintable(Some(&texture));
        }

        self.wallpaper.set_content_fit(gtk4::ContentFit::Cover);
        self.wallpaper.set_overflow(gtk4::Overflow::Hidden);
        self.window.set_child(Some(&self.wallpaper));
        self.window.present();
    }

    fn get_geometry(&self) -> (i32, i32) {
        let display =
            gtk4::gdk::Display::default().expect("Couldnt find a valid monitor or display !");
        let monitors = display.monitors();
        if let Some(monitor) = monitors
            .item(0)
            .and_then(|m| m.downcast::<gtk4::gdk::Monitor>().ok())
        {
            let geometry = monitor.geometry();
            let screen_width = geometry.width();
            let screen_height = geometry.height();

            (screen_width, screen_height)
        } else {
            (1920, 1080)
        }
    }

    fn get_wallpaper() -> gtk4::glib::Bytes {
        let bytes = std::fs::read("/home/kaif/wallpaper.png").expect("Failed to read wallpaper file");
        gtk4::glib::Bytes::from(&bytes)
    }
}
