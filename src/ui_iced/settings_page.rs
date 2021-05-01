use iced::{Button, Column, Element, Length, Row, Space, Text};

use crate::locale::InterfaceSymbol;

use super::{common_elements::COLUMN_SPACING, MainApp, Message, Page};

pub trait SettingsPage {
    fn get_settings_page(&mut self) -> Element<Message>;
}

impl SettingsPage for MainApp {
    fn get_settings_page(&mut self) -> Element<Message> {
        let mut locales_choice = Column::new().spacing(COLUMN_SPACING);

        for (locale_name, state) in self
            .locales
            .keys()
            .zip(self.state_buttons_locale.iter_mut())
        {
            let button = Button::new(state, Text::new(locale_name));
            locales_choice = locales_choice.push(if locale_name != &self.selected_locale {
                button.on_press(Message::LocaleChanged(locale_name.clone()))
            } else {
                button
            });
        }

        Column::new()
            .padding(5)
            .push(
                Row::new()
                    .push(Space::with_width(Length::Fill))
                    .push(locales_choice)
                    .height(Length::Fill),
            )
            .push(
                Row::new().push(Space::with_width(Length::Fill)).push(
                    Button::new(
                        &mut self.state_settings_button,
                        Text::new(InterfaceSymbol::Back),
                    )
                    .on_press(Message::ChangePage(Page::Main)),
                ),
            )
            .into()
    }
}
