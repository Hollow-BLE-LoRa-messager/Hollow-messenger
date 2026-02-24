slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    main_window.on_text_changed({
    let main_window_handle = main_window.as_weak();
    move |new_text| {
        let main_window = main_window_handle.unwrap();
        let limited: String = new_text.chars().take(50).collect();
        main_window.set_search_text(limited.into());
    }
});

    main_window.run()
}