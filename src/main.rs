mod app;
mod handlers;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}