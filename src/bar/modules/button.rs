use kdlite::dom::Document;
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
}
