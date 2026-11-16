#[cfg(not(target_os = "android"))]
pub mod menu;
#[cfg(not(target_os = "android"))]
pub use menu::{Menu, Submenu};

#[cfg(target_os = "android")]
pub mod menu_types;
#[cfg(target_os = "android")]
pub use menu_types::{Menu, Submenu, MenuItem, MenuId};
