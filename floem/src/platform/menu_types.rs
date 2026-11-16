//! Platform-agnostic menu types.
//!
//! This module provides unified menu types that work across all platforms,
//! using muda on native platforms and wasm_stubs on wasm32.

#[cfg(not(target_os = "android"))]
pub use muda::{
    accelerator::Accelerator,
    CheckMenuItem, Icon, IconMenuItem, IsMenuItem, Menu, MenuId, MenuItem, NativeIcon,
    PredefinedMenuItem, Submenu,
};
#[cfg(not(target_os = "android"))]
pub use muda::MenuItemKind;

#[cfg(target_os = "android")]
pub type MenuId = String;

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Menu;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct MenuItem;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct MenuItemKind;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Submenu;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct PredefinedMenuItem;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct CheckMenuItem;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct IconMenuItem;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Icon;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct NativeIcon;
#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Accelerator;

// 👇 这里是核心修复：把 struct 改成 trait
#[cfg(target_os = "android")]
pub trait IsMenuItem: std::fmt::Debug + Send + Sync {}

#[cfg(target_os = "android")]
impl<T: std::fmt::Debug + Send + Sync> IsMenuItem for T {}

#[cfg(target_os = "android")]
impl Menu {
    pub fn new() -> Self { Self }
}
#[cfg(target_os = "android")]
impl Submenu {
    pub fn new() -> Self { Self }
}

#[cfg(target_os = "android")]
impl Menu {
    pub fn new() -> Self { Self }
}
#[cfg(target_os = "android")]
impl Submenu {
    pub fn new() -> Self { Self }
}
