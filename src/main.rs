mod armor_and_skills;
mod build_search;
mod locale;
mod style_iced;
mod ui_iced;

use std::sync::Mutex;

use iced::{Sandbox, Settings};
use locale::Locale;
use once_cell::sync::Lazy;

const FONT: &[u8] = include_bytes!("fonts/FiraSans-Regular.ttf");
// I use this global variable to use the locale within the Display trait
// for the Skill struct. If you have another solution don't hesitate to tell me.
static LOCALE: Lazy<Mutex<Option<Locale>>> = Lazy::new(Default::default);

pub fn main() -> iced::Result {
    ui_iced::MainApp::run(Settings {
        default_font: Some(FONT),
        ..Default::default()
    })
}
