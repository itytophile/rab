use iced::{pure, Length};

use super::{
    common_elements::{update_button, ICON_SIZE},
    MainApp, Msg,
};

pub trait NoFilesPage {
    fn get_no_files_page(&self) -> pure::widget::Button<Msg>;
}

impl NoFilesPage for MainApp {
    fn get_no_files_page(&self) -> pure::widget::Button<Msg> {
        update_button(self.update_state, Msg::DownloadArmors).height(Length::Units(ICON_SIZE))
    }
}
