use crate::AppWindow;

pub fn setup(window: &AppWindow) {
    window.on_close_window(move || {
        slint::quit_event_loop().unwrap();
    });
}