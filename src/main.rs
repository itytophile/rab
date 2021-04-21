mod armor_ron;
mod build_search;
mod style_iced;
mod ui_iced;

use iced::{Sandbox, Settings};

const FONT: &[u8] = include_bytes!("fonts/FiraSans-Regular.ttf");

pub fn main() -> iced::Result {
    ui_iced::MainApp::run(Settings {default_font: Some(FONT),..Default::default()})
}
