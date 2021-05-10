use iced::{Element, Length};

use super::{MainApp, Msg, common_elements::{ICON_SIZE, update_button}};

pub trait NoFilesPage {
    fn get_no_files_page(&mut self) -> Element<Msg>;
}

impl NoFilesPage for MainApp {
    fn get_no_files_page(&mut self) -> Element<Msg> {
        update_button(&mut self.state_update_button, self.update_state, Msg::DownloadArmors)
            .height(Length::Units(ICON_SIZE))
            .into()
    }
}
