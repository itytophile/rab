use iced::{Align, Button, Column, Container, Element, Length, Row, Space, Text};

use crate::locale::InterfaceSymbol;

use super::{common_elements::COLUMN_SPACING, MainApp, Msg, Page};

pub trait LangPage {
    fn get_lang_page(&mut self) -> Element<Msg>;
}

impl LangPage for MainApp {
    fn get_lang_page(&mut self) -> Element<Msg> {
        let mut locales_choice = Column::new().spacing(COLUMN_SPACING);

        for (locale_name, state) in self
            .locales
            .keys()
            .zip(self.state_buttons_locale.iter_mut())
        {
            let button = Button::new(
                state,
                Container::new(Text::new(locale_name))
                    .width(Length::Units(100))
                    .center_x(),
            );
            locales_choice = locales_choice.push(if locale_name != &self.selected_locale {
                button.on_press(Msg::LocaleChanged(locale_name.clone()))
            } else {
                button
            });
        }

        Column::new()
            .align_items(Align::Center)
            .padding(5)
            .push(
                Container::new(locales_choice)
                    .height(Length::Fill)
                    .center_y(),
            )
            .push(
                Row::new().push(Space::with_width(Length::Fill)).push(
                    Button::new(
                        &mut self.state_lang_button,
                        Text::new(InterfaceSymbol::Back),
                    )
                    .on_press(Msg::ChangePage(Page::Main)),
                ),
            )
            .into()
    }
}
