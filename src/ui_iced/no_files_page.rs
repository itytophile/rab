use iced::{Element, Length};

use super::{common_elements::update_button, MainApp, Msg};

pub trait NoFilesPage {
    fn get_no_files_page(&mut self) -> Element<Msg>;
}

impl NoFilesPage for MainApp {
    fn get_no_files_page(&mut self) -> Element<Msg> {
        update_button(&mut self.state_update_button, self.update_state, Msg::DownloadArmors)
            .height(Length::Units(30))
            .into()
    }
}
