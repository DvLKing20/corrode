use kdl::KdlDocument;
use smallvec::SmallVec;
use std::path::PathBuf;

enum Icon<'a> {
    File(PathBuf),
    Text(&'a str),
}

pub struct ModuleConfig<'a> {
    icon: Icon<'a>,
    class: &'a str,
    action: &'a str,
}

impl<'a> ModuleConfig<'a> {
    pub fn new(icon: Icon<'a>, class: &'a str, action: &'a str) -> Self {
        Self {
            icon,
            class,
            action,
        }
    }

    pub fn parse_config_to_modules(
        doc: &'a KdlDocument,
    ) -> Option<SmallVec<[ModuleConfig<'a>; 8]>> {
        let mut modules_vec: SmallVec<[ModuleConfig; 8]> = SmallVec::new();
        let exts = [
            ".svg", ".png", ".jpg", ".jpeg", ".webp", ".gif", ".bmp", ".ico", ".avif",
        ];

        let modules_children = doc
            .get("bar")
            .and_then(|bar| bar.children())
            .and_then(|children| children.get("modules"))
            .and_then(|modules| modules.children())?;

        for node in modules_children.nodes() {
            if node.name().value() == "button" {
                if let (Some(class), Some(icon_str), Some(action)) = (
                    node.get("name").and_then(|e| e.as_string()),
                    node.get("icon").and_then(|e| e.as_string()),
                    node.get("action").and_then(|e| e.as_string()),
                ) {
                    let icon = if exts.iter().any(|ext| icon_str.ends_with(ext)) {
                        Icon::File(PathBuf::from(icon_str))
                    } else {
                        Icon::Text(icon_str)
                    };

                    modules_vec.push(Self::new(icon, class, action));
                } else {
                    return None;
                }
            }
        }
        Some(modules_vec)
    }
}
