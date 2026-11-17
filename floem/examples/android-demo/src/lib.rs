//! Minimal `cdylib` entry point used to verify that floem renders on
//! `aarch64-linux-android`.
//!
//! `android-activity` looks up `android_main()` once this library is loaded and
//! hands over the `AndroidApp`. That handle has to reach floem *before* it
//! builds its event loop, otherwise winit panics.

#[cfg(target_os = "android")]
use floem::platform::android::AndroidApp;
#[cfg(target_os = "android")]
use floem::prelude::*;

#[cfg(target_os = "android")]
fn app_view() -> impl IntoView {
    label(|| "Hello from Floem on Android".to_string())
        .style(|s| s.size_full().items_center().justify_center().font_size(24.0))
}

/// Entry point invoked by the `android-activity` glue.
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    floem::platform::android::set_android_app(app);
    floem::launch(app_view);
}
