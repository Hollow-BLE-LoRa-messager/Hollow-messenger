use i_slint_backend_winit::WinitWindowAccessor;

use slint::ComponentHandle;
use crate::AppWindow;

pub fn setup(window: &AppWindow) {
    let handle = window.as_weak();

    window.on_toggle_maximize(move || {
        let app = handle.unwrap();
        let is_max = app.get_is_maximized();

        if is_max {
            app.set_target_width(800.0);
            app.set_target_height(600.0);
            app.set_is_maximized(false);
        } else {
            app.window().with_winit_window(|win| {
                if let Some(monitor) = win.current_monitor() {
                    let size = monitor.size();
                    let scale = win.scale_factor() as f32;
                    app.set_target_width(size.width as f32 / scale);
                    app.set_target_height(size.height as f32 / scale);
                }
            });
            app.set_is_maximized(true);
        }
    });
}