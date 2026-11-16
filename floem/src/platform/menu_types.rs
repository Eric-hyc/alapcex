#[cfg(not(target_os = "android"))]
pub use muda::{Menu, Submenu, MenuItem, MenuId, MenuItemKind, PredefinedMenuItem, CheckMenuItem, IconMenuItem};

#[cfg(target_os = "android")]
pub type MenuId = String;

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Menu;
#[cfg(target_os = "android")]
impl Menu {
    pub fn new() -> Self { Self }
    pub fn append(&self, _item: &dyn std::fmt::Debug) -> Result<(), ()> { Ok(()) }
    pub fn append_items(&self, _items: &[&dyn std::fmt::Debug]) -> Result<(), ()> { Ok(()) }
    pub fn prepend(&self, _item: &dyn std::fmt::Debug) -> Result<(), ()> { Ok(()) }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct Submenu;
#[cfg(target_os = "android")]
impl Submenu {
    pub fn new(_text: &str) -> Self { Self }
    pub fn append(&self, _item: &dyn std::fmt::Debug) -> Result<(), ()> { Ok(()) }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct MenuItem;
#[cfg(target_os = "android")]
impl MenuItem {
    pub fn new(_text: &str, _accelerator: Option<String>) -> Self { Self }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct PredefinedMenuItem;
#[cfg(target_os = "android")]
impl PredefinedMenuItem {
    pub fn separator() -> Self { Self }
    pub fn copy() -> Self { Self }
    pub fn cut() -> Self { Self }
    pub fn paste() -> Self { Self }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct CheckMenuItem;
#[cfg(target_os = "android")]
impl CheckMenuItem {
    pub fn new(_text: &str, _checked: bool, _accelerator: Option<String>) -> Self { Self }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub struct IconMenuItem;
#[cfg(target_os = "android")]
impl IconMenuItem {
    pub fn new(_text: &str, _icon: Option<String>, _accelerator: Option<String>) -> Self { Self }
}

#[cfg(target_os = "android")]
#[derive(Clone, Debug)]
pub enum MenuItemKind {
    Item,
    Submenu,
    Predefined,
    Check,
    Icon,
}

#[cfg(target_os = "android")]
impl Menu {
    pub fn build(self) -> Self { self }
}

#[cfg(target_os = "android")]
impl std::fmt::Display for MenuItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
