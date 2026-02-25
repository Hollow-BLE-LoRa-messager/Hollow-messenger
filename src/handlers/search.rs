use slint::ComponentHandle;
use crate::AppWindow;

pub fn setup(window: &AppWindow) {
    let handle = window.as_weak();

    window.on_text_changed(move |new_text| {
        let app = handle.unwrap();
        let limited: String = new_text.chars().take(50).collect();
        app.set_search_text(limited.into());
    });
}