use kdl::KdlDocument;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{PathBuf},
};

pub fn parse_config() -> KdlDocument {
    let Ok(home) = std::env::var("HOME") else {
        write_to_log(
            &mut PathBuf::from("."),
            "ERROR",
            "HOME var isnt set! Dropping The logging in The Current Dir",
        );
        return KdlDocument::new();
    };

    let mut config_path: PathBuf = PathBuf::from(home);
    config_path.push(".config/shell");

    if !config_path.exists() {
        let Ok(_) = fs::create_dir_all(&config_path) else {
            write_to_log(
                &mut config_path,
                "WARN",
                "Failed to create configuration directory!.",
            );
            return KdlDocument::new();
        };
    };

    config_path.push("config.kdl");

    if !config_path.exists() {
        let default_config = r#"
            bar{
               position "top"
               opacity 0.85
               orientation "Horizontal"

               modules {
                  button name="foot" icon="/usr/share/icons/hicolor/48x48/apps/foot.svg" exec="foot &"
               }
            }
            "#;
        if fs::write(&config_path, default_config).is_err() {
            write_to_log(&mut config_path, "WARN", "Failed to write default config.kdl layout to disk.");
            return KdlDocument::new();
        };
    };

    let Ok(content) = fs::read_to_string(&config_path) else {
        write_to_log(&mut config_path, "ERROR", "Target configuration file is unreadable.");
        return KdlDocument::new();
    };

    let Ok(doc) = KdlDocument::parse(&content) else {
            write_to_log(&mut config_path, "ERROR", "KDL Syntax Parser Error inside config.kdl!");
            return KdlDocument::new();
    };

    doc
}

fn write_to_log(log_path: &mut PathBuf, log_level: &str, msg: &str) {
    
    if log_path.is_absolute() && (log_path.is_file() || log_path.extension().is_some()) {
        log_path.pop();
    }

    log_path.push("config.log");

    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) else {
        return;
    };

    let _ = file.write_all(b"[");
    let _ = file.write_all(log_level.as_bytes());
    let _ = file.write_all(b"] ");
    let _ = file.write_all(msg.as_bytes());
    let _ = file.write_all(b"\n");
}
