use std::{
    fs::{self},
    path::PathBuf,
};

pub struct CorrodeNodes<'a> {
    pub bar: Option<kdlite::dom::Node<'a>>,
}

impl<'a> CorrodeNodes<'a> {
    pub fn new(content: Option<&'a str>) -> Self {
        let mut bar = None;

        let Some(content) = content else {
            return Self { bar }
        };

        let doc = match kdlite::dom::Document::parse(content) {
            Ok(doc) => doc,
            Err(e) => {
                log_error!(
                    "KDL Syntax Parser Error inside config.kdl! Details: {:?}",
                    e
                );
                return Self { bar };
            }
        };

        for node in doc.nodes {
            match node.name() {
                "bar" => bar = Some(node),
                _ => {}
            }
        }

        Self { bar }
    }

    pub fn parse_content() -> Option<String> {
        let Ok(home) = std::env::var("HOME") else {
            log_warn!("Home Variable Isnt Set! Make Sure to Add it.");
            return None;
        };

        let mut config_path: PathBuf = PathBuf::from(home);
        config_path.push(".config/Corrode");

        if !config_path.exists() {
            let Ok(_) = fs::create_dir_all(&config_path) else {
                log_warn!("Failed to create configuration directory!.");
                return None;
            };
        };

        config_path.push("config.kdl");

        if !config_path.exists() {
            let default_config = r#"
bar class-name = "corrode-bar" {
    Width -1
    Height 30
    Exclusive 30
    Position "top"
    Opacity 0.85
    Layer "top"

    Clock {
       
    }

    Cpu {


    }
}
            "#;
            if fs::write(&config_path, default_config).is_err() {
                log_warn!("Failed to write default config.kdl layout to disk.");
                return None;
            };
        };

        let Ok(content) = fs::read_to_string(&config_path) else {
            log_warn!("Target configuration file is unreadable.");
            return None;
        };

        Some(content)
    }
}
