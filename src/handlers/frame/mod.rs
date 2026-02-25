mod drag;
mod close;
mod maximize;

use crate::AppWindow;

pub fn setup(window: &AppWindow) {
    drag::setup(window);
    close::setup(window);
    maximize::setup(window);
}