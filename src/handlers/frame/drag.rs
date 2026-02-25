use i_slint_backend_winit::WinitWindowAccessor;

use slint::ComponentHandle;
use crate::AppWindow;

pub fn setup(window: &AppWindow) {
    let handle = window.as_weak();

    let app = handle.unwrap();
    if app.get_is_maximized() {
        app.set_target_width(800.0);
        app.set_target_height(600.0);
        app.set_is_maximized(false);
    }

    window.on_start_drag(move || {
        handle.unwrap().window().with_winit_window(|win| {
            let _ = win.drag_window();
        });
    });
}