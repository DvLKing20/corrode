use gtk4::gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4_layer_shell::{Edge, Layer};
use shell::wallpaper::wallpaper::WallpaperConfig;
use shell::{bar::bar::Bar, config};
const ID: &str = "org.custom.shell";

fn main() {
    let app = gtk4::Application::builder().application_id(ID).build();

    app.connect_activate(|app| {
        let bar = Bar::new(app);
        let wallpaper = WallpaperConfig::new(app);

        wallpaper.init();
        bar.create_bar();
        bar.display_bar();
        config::parse_config();
    });

    app.run();
}
