use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use kdl::KdlDocument;

use crate::config::parse_config;

pub struct Bar {
    window: gtk4::ApplicationWindow,
    container: gtk4::Box,
    config: BarConfig
}

impl Bar {
    pub fn new(app: &gtk4::Application, doc: &KdlDocument) -> Bar {
        let window = gtk4::ApplicationWindow::new(app);
        window.add_css_class("bar");
 
        let config = match Self::parse_bar_config(doc) {
            Some(config) => config,
            None => {
                       


               };
            };
        };

        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        Self { window, container, config }
    }

    fn parse_bar_config(doc: &KdlDocument) -> Option<BarConfig>{
        
        let modules_children = doc
            .get("bar")
            .and_then(|bar| bar.children())?;
    }

    fn apply_style(&self) {
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(
            "
     .bar{  
            background-color: rgba(30, 30, 46, 0.9);
            border-radius: 15px;
            border: 1px solid #585b70;
     }
     button {
           min-height: 16px;
           min-width:  16px;
           border-radius: 10px;
     }

     ",
        );

        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        };
    }

    fn add_buttons(&self, icon: &str, class_name: &str) {
        let btn = gtk4::Button::with_label(icon);
        btn.add_css_class(class_name);
        btn.connect_clicked(|_| println!("Button Clicked"));

        btn.set_hexpand(false);
        btn.set_vexpand(false);
        btn.set_valign(gtk4::Align::Center);
        btn.set_halign(gtk4::Align::Center);

        btn.set_size_request(16, 16);

        self.container.append(&btn);
    }

    pub fn create_bar(&self) {
        self.window.init_layer_shell();
        self.window.set_layer(self.layer);

        self.window.set_default_size(0, self.exclusive_zone);

        for anchor in self.anchors {
            self.window.set_anchor(anchor, true);
        }
        self.window.set_margin(Edge::Top, 10);
        self.window.set_margin(Edge::Right, 300);
        self.window.set_margin(Edge::Left, 300);
        self.window.set_opacity(self.opacity);
        self.window.set_exclusive_zone(self.exclusive_zone);

        self.window.set_child(Some(&self.container));

        self.add_buttons("", "btn_1");
    }

    pub fn display_bar(&self) {
        self.apply_style();
        self.window.present();
    }
}
