//! Android startup glue.
//!
//! `android-activity` (pulled in through winit's `android-native-activity`
//! feature) calls `android_main()` in the `cdylib` once the library is loaded
//! and hands over the [`AndroidApp`]. Floem, however, creates its `EventLoop`
//! deep inside `Application::new_with_config()`, so the handle has to be
//! stashed here first and consumed when the event loop is built.
//!
//! Without this, winit panics with:
//! `An AndroidApp as passed to android_main() is required to create an EventLoop on Android`.

use std::cell::RefCell;

/// The `android-activity` handle that winit needs to create an event loop.
///
/// Re-exported so that the `cdylib` entry point does not have to depend on
/// `floem-winit` directly, which also guarantees both sides use the very same
/// crate instance.
pub use winit::platform::android::activity::AndroidApp;

thread_local! {
    static ANDROID_APP: RefCell<Option<AndroidApp>> = RefCell::new(None);
}

/// Store the [`AndroidApp`] handed to `android_main()`.
///
/// Must be called before `floem::launch()` / `Application::new()`, otherwise
/// event loop creation panics.
pub fn set_android_app(app: AndroidApp) {
    ANDROID_APP.with(|slot| *slot.borrow_mut() = Some(app));
}

/// Take the stored [`AndroidApp`], if any.
///
/// Consumed exactly once, when the event loop is constructed.
pub(crate) fn take_android_app() -> Option<AndroidApp> {
    ANDROID_APP.with(|slot| slot.borrow_mut().take())
}
