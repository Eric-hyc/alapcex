use super::file::{FileDialogOptions, FileInfo};

pub fn open_file(
    _options: FileDialogOptions,
    file_info_action: impl Fn(Option<FileInfo>) + 'static,
) {
    file_info_action(None);
}

pub fn save_as(
    _options: FileDialogOptions,
    file_info_action: impl Fn(Option<FileInfo>) + 'static,
) {
    file_info_action(None);
}
