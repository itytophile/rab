use iced::{Element, HorizontalAlignment, Length, Text, VerticalAlignment};

use super::Message;

pub fn get_error_page<'a>(msg: String) -> Element<'a, Message> {
    Text::new(format!(
        "There is an error with the armor files.\n\n\
        Did you forget to download the \"armors\" folder?\nThis folder must be next to the executable.\n\n\
        Check the README at\nhttps://github.com/itytophile/rab#readme\n\nError: {}",
        msg
    )).width(Length::Fill).height(Length::Fill)
    .horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center).into()
}
