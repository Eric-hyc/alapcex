use std::fmt;

#[derive(Debug)]
pub struct ClipboardError;
impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Clipboard error") }
}
impl std::error::Error for ClipboardError {}

pub struct Clipboard;

#[cfg(target_os = "android")]
impl Clipboard {
    pub fn new() -> Self { Self }
    pub fn init(_raw: raw_window_handle::RawDisplayHandle) {}
    pub fn get_contents() -> Result<String, ClipboardError> { Ok(String::new()) }
    pub fn set_contents(_content: String) -> Result<(), ClipboardError> { Ok(()) }
    pub fn replace_region(_content: &str) -> Result<(), ClipboardError> { Ok(()) }
    pub fn is_empty() -> bool { true }
    pub fn clear() {}
}
