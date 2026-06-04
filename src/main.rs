use corrode::{
    bar::core::Core,
    config::parser::CorrodeNodes,
    wallpaper::{background::Wallpaper, picker::Picker},
};
use gtk4::{
    gio::prelude::{ApplicationCommandLineExt, ApplicationExt, ApplicationExtManual},
    glib::{self},
};

const ID: &str = "org.corrode.Shell";

static mut WALLPAPER: *mut Wallpaper = std::ptr::null_mut();

fn main() {
    let _= gtk4::init();

    let app = gtk4::Application::builder()
        .application_id(ID)
        .flags(gtk4::gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_command_line(move |app, cmd| {
        let args = cmd.arguments();
        let app_clone = app.clone();

        let is_picker = args.iter().any(|arg| arg.as_os_str() == "--picker");

        if is_picker {
            unsafe {
                let wallpaper = WALLPAPER;

                if !wallpaper.is_null() {
                    gtk4::glib::idle_add_local(move || {
                        let picker = Box::into_raw(Box::new(Picker::new(&app_clone)));
                        Picker::setup_controls(picker, wallpaper);
                        if let Some(file) = (*picker).gallery.next() {
                            (*picker).update(file, gtk4::StackTransitionType::None, wallpaper);
                        }
                        (*picker).init();
                        glib::ControlFlow::Break
                    });
                };
            }
        }
        else {
            app.activate();
        }
        gtk4::glib::ExitCode::SUCCESS
    });

    app.connect_activate(move |app| {
        let mut wallpaper = Wallpaper::new();
        wallpaper.init(app);
        let ptr = Box::into_raw(Box::new(wallpaper));
        unsafe { WALLPAPER = ptr }
        let content = CorrodeNodes::parse_content();
        let nodes = CorrodeNodes::new(content.as_deref());
        let bar = Core::new(app, nodes.bar.as_ref());
        bar.init();
    });

    app.run();
}
