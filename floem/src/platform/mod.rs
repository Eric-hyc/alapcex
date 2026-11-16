pub mod action;
pub mod clipboard;
pub mod file;
pub mod file_action;
pub use std::time::{Duration, Instant};

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
