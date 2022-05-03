use super::{common_elements::COLUMN_SPACING, MainApp, Msg, Page};
use crate::locale::InterfaceSymbol;
use iced::{pure, Alignment, Length, Space, Text};

pub trait LangPage {
    fn get_lang_page(&self) -> pure::widget::Column<Msg>;
}

impl LangPage for MainApp {
    fn get_lang_page(&self) -> pure::widget::Column<Msg> {
        let mut locales_choice = pure::column().spacing(COLUMN_SPACING);

        for locale_name in self.locales.keys() {
            let button = pure::button(
                pure::container(Text::new(locale_name))
                    .width(Length::Units(100))
                    .center_x(),
            );
            locales_choice = locales_choice.push(if locale_name != &self.selected_locale {
                button.on_press(Msg::LocaleChanged(locale_name.clone()))
            } else {
                button
            });
        }

        pure::column()
            .align_items(Alignment::Center)
            .padding(5)
            .push(
                pure::container(locales_choice)
                    .height(Length::Fill)
                    .center_y(),
            )
            .push(
                pure::row().push(Space::with_width(Length::Fill)).push(
                    pure::button(Text::new(InterfaceSymbol::Back))
                        .on_press(Msg::ChangePage(Page::Main)),
                ),
            )
    }
}
