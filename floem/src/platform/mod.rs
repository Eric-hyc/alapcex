pub mod context_menu;
pub mod file;
pub mod file_action;
pub mod time;
pub mod wasm_stubs;

pub use std::time::{Duration, Instant};
pub use clipboard::{Clipboard, ClipboardError};
pub use file::{FileDialogOptions, FileInfo, FileSpec};
pub use file_action::{open_file, save_as};

#[cfg(not(target_os = "android"))]
pub mod menu;
#[cfg(not(target_os = "android"))]
pub use menu::{Menu, Submenu};

#[cfg(target_os = "android")]
pub mod menu_types;
#[cfg(target_os = "android")]
pub use menu_types as menu;
#[cfg(target_os = "android")]
pub use menu_types::{Menu, Submenu};

pub mod clipboard;
