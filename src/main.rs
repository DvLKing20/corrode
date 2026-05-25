use gtk4::gio::prelude::{ApplicationExt, ApplicationExtManual};
use corrode::{bar::{self, core::Core}, config::parser::CorrodeNodes, log_warn, wallpaper::{self, wallpaper::WallpaperConfig}};
const ID: &str = "org.corrode.Shell";

fn main() {
    let app = gtk4::Application::builder().application_id(ID).build();
    app.connect_activate(|app| {

        let content = CorrodeNodes::parse_content();
        let nodes = CorrodeNodes::new(content.as_deref());

        let bar = Core::new(app, nodes.bar.as_ref());
        drop(nodes);
        drop(content);
        WallpaperConfig::new(app).init();
        bar.init();
    });

    app.run();
}


