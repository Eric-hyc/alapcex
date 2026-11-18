//! Lapce Android Entry Point
//! This crate acts as the cdylib glue between Android NativeActivity and Lapce.

#[cfg(target_os = "android")]
use floem::platform::android::AndroidApp;
#[cfg(target_os = "android")]
use floem::prelude::*;

#[cfg(target_os = "android")]
fn lapce_android_view() -> impl IntoView {
    // Temporarily render a placeholder to verify Lapce-linked-Floem works
    label(|| "Lapce Android Loading...".to_string())
        .style(|s| s.size_full().items_center().justify_center().font_size(32.0))
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: AndroidApp) {
    floem::platform::android::set_android_app(app);
    // In the future, we will call lapce::run() here
    floem::launch(lapce_android_view);
}
