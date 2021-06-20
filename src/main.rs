mod locale;
mod profile;
mod style_iced;
mod ui_iced;
mod update;

use std::sync::Mutex;

use iced::{Application, Settings};
use locale::Locale;
use once_cell::sync::Lazy;

const FONT: &[u8] = include_bytes!("fonts/FiraSans-Regular.ttf");
// I use this global variable to use the locale within the implementations of the Display trait
// If you have another solution don't hesitate to tell me.
static LOCALE: Lazy<Mutex<Option<Locale>>> = Lazy::new(Default::default);

const ARMORS_PATH: &str = "armors";
const LOCALE_DIR_PATH: &str = "locale";

pub fn main() -> iced::Result {
    ui_iced::MainApp::run(Settings {
        default_font: Some(FONT),
        ..Default::default()
    })
}
