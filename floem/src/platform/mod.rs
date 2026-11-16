pub mod clipboard;
pub mod context_menu;
pub mod file;
pub mod file_action;
pub mod time;
pub mod wasm_stubs;

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

// 重新导出公用的类型，解决 lib.rs 里的报错
pub use clipboard::{Clipboard, ClipboardError};
pub use file::{FileDialogOptions, FileInfo, FileSpec};
