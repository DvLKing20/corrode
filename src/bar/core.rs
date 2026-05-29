use crate::bar::config::BarConfig;
use gtk4::prelude::{GtkWindowExt, WidgetExt};
use gtk4_layer_shell::LayerShell;

pub struct Core<'a> {
    window: gtk4::ApplicationWindow,
    config: BarConfig,
    class_name: &'a str,
}

impl<'a> Core<'a> {
    pub fn default(app: &gtk4::Application) -> Self {
        Self {
            window: gtk4::ApplicationWindow::new(app),
            config: BarConfig::default(),
            class_name: "corrode-bar",
        }
    }

    pub fn new(app: &gtk4::Application, node: Option<&'a kdlite::dom::Node>) -> Self {
        let window = gtk4::ApplicationWindow::new(app);

        let class_name = node
            .and_then(|n| n.entries.first())
            .and_then(|e| match &e.value {
                kdlite::dom::Value::String(v) => Some(v.as_ref()),
                _ => None,
            }).unwrap_or("corrode-bar");

        let child = match node.and_then(|n| n.children.as_ref()) {
            Some(doc) => doc,
            None => return Core::default(app),
        };

        let mut stream = child.nodes.iter().peekable();

        let config = BarConfig::new(&mut stream);

        Self {
            window,
            config,
            class_name,
        }
    }

    pub fn init(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(self.config.layer);
        for anchor in self.config.anchors {
            self.window.set_anchor(anchor, true);
        }
        self.window.add_css_class(self.class_name);
        self.window.set_default_size(self.config.width, self.config.exclusive_zone);
        self.window.set_opacity(self.config.opacity);
        self.window.set_exclusive_zone(self.config.exclusive_zone);

        self.window.present();
    }
}
