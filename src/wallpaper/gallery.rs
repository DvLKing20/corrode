use crate::EXTENSIONS;
    
use gtk4::{Picture, gio};
use smallvec::SmallVec;
use std::{ffi::OsStr, path::PathBuf};

pub struct Gallery {
    wallpaper: Picture,
    files: Option<SmallVec<[gio::File; 8]>>,
    index: usize,
}

impl Gallery {
    pub fn new() -> Self {
        let wallpaper = Picture::new();
        let files = Self::get_wallpaper();
        let index = 0;

        Self {
            wallpaper,
            files,
            index,
        }
    }

    pub fn next(&mut self) -> Option<&gio::File> {
        let files = self.files.as_ref()?;

        if files.is_empty() {
            log_warn!("There Was No Wallpapers Or the Dir was Empty");
            return None;
        };

        self.index = (self.index + 1) % files.len();
        Some(&files[self.index])
    }

    pub fn prev(&mut self) -> Option<&gio::File> {
        let files = self.files.as_ref()?;

        if files.is_empty() {
            return None;
        };

        self.index = (self.index + files.len() - 1) % files.len();
        Some(&files[self.index])
    }

    pub fn get_wallpaper() -> Option<SmallVec<[gio::File; 8]>> {
        let mut vec: SmallVec<[gio::File; 8]> = SmallVec::new();

        let pictures_path = std::env::var("XDG_PICTURES_DIR")
            .map_err(|_| log_warn!("XDG_PICTURES_DIR missing, trying $HOME..."))
            .or_else(|_| std::env::var("HOME").map_err(|_| log_error!("$HOME also missing!")))
            .map(|p| PathBuf::from(p).join("Pictures/wallpapers"))
            .ok()?;

        let entries = match pictures_path.read_dir() {
            Ok(v) => v,
            Err(e) => {
                log_error!("{:?}", e);
                return None;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => continue,
            };

            let Ok(file) = entry.file_type() else {
                continue;
            };

            if file.is_file() {
                let img_path = entry.path();
                let img = img_path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|ext| EXTENSIONS.iter().any(|e| ext.eq_ignore_ascii_case(e)))
                    .unwrap_or(false);

                if img {
                    let file = gio::File::for_path(entry.path());
                    vec.push(file);
                }
            };
        }

        Some(vec)
    }
}
