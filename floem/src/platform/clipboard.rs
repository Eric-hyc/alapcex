use std::fmt;

#[derive(Debug)]
pub struct ClipboardError;
impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clipboard error")
    }
}
impl std::error::Error for ClipboardError {}

pub struct Clipboard;

#[cfg(not(target_os = "android"))]
impl Clipboard {
    pub fn init(_raw: raw_window_handle::RawDisplayHandle) {
        // 桌面端原本的逻辑（先注释掉，防止依赖问题，若报错再恢复）
    }
    pub fn is_empty(&self) -> bool { true }
    pub fn get_contents(&self) -> Result<String, ClipboardError> { Ok(String::new()) }
    pub fn replace_region(&self, _content: &str) -> Result<(), ClipboardError> { Ok(()) }
    pub fn set_contents(&self, _content: String) -> Result<(), ClipboardError> { Ok(()) }
}

#[cfg(target_os = "android")]
impl Clipboard {
    pub fn init() {
        // Android 暂不需要初始化
    }
    pub fn is_empty(&self) -> bool { true }
    pub fn get_contents(&self) -> Result<String, ClipboardError> { Ok(String::new()) }
    pub fn replace_region(&self, _content: &str) -> Result<(), ClipboardError> { Ok(()) }
    pub fn set_contents(&self, _content: String) -> Result<(), ClipboardError> { Ok(()) }
}
