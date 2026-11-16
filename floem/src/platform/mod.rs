#[cfg(not(target_os = "android"))]
pub mod menu;

// 魔法在这里：在 Android 下，把 menu_types 模块重命名为 menu
#[cfg(target_os = "android")]
pub mod menu_types;
#[cfg(target_os = "android")]
pub use menu_types as menu;

pub mod action;
pub mod clipboard;
pub mod file;
pub mod file_action;
