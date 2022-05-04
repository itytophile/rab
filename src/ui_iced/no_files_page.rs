use super::{
    common_elements::{update_button, ICON_LENGTH},
    MainApp, Msg,
};
use iced::pure;

pub trait NoFilesPage {
    fn get_no_files_page(&self) -> pure::widget::Button<Msg>;
}

impl NoFilesPage for MainApp {
    fn get_no_files_page(&self) -> pure::widget::Button<Msg> {
        update_button(self.update_state, Msg::DownloadArmors).height(ICON_LENGTH)
    }
}
