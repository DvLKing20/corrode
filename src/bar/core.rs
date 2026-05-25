use crate::bar::config::BarConfig;
            
use gtk4::prelude::{GtkWindowExt, WidgetExt};
use gtk4_layer_shell::LayerShell;

pub struct Core {
    window: gtk4::ApplicationWindow,
    config: BarConfig,
}

impl Core {
    pub fn default(app: &gtk4::Application) -> Self {
        Self {
            window: gtk4::ApplicationWindow::new(app),
            config: BarConfig::default(),
        }
    }

    pub fn new(app: &gtk4::Application, node: Option<&kdlite::dom::Node>) -> Self {
        let window = gtk4::ApplicationWindow::new(app);

        let child = match node.and_then(|n| n.children.as_ref()) {
            Some(doc) => doc,
            None => return Core::default(app),
        };

        let mut stream = child.nodes.iter().peekable();

        let config = BarConfig::new(&mut stream);

        Self { window, config }
    }

    pub fn init(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(self.config.layer);
        for anchor in self.config.anchors {
            self.window.set_anchor(anchor, true);
        }
        self.window.set_default_size(-1, self.config.exclusive_zone);
        self.window.set_opacity(self.config.opacity);
        self.window.set_exclusive_zone(self.config.exclusive_zone);

        self.window.present();
    }
}
