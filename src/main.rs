mod armor_ron;
mod build_search;
mod style_iced;
mod ui_iced;

use iced::{Sandbox, Settings};

pub fn main() -> iced::Result {
    ui_iced::MainApp::run(Settings::default())
}
