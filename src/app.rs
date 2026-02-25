use slint::ComponentHandle;
use crate::handlers;
use crate::AppWindow;

pub fn run() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;

    // handlers::frame::setup(&window);
    handlers::search::setup(&window);

    window.run()
}