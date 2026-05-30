#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[33m[WARN]\x1b[0m {}", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31m[ERROR]\x1b[0m {}", format_args!($($arg)*));
    }};
}

const EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "tiff", "svg"];

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod bar;
pub mod config;
pub mod monitors;
pub mod wallpaper;
