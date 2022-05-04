use super::{
    common_elements::{
        armor_desc_to_element, get_column_builds_found, get_skill_filter, get_wishfield_row,
        update_button, BUTTON_SPACING, COLUMN_SPACING, FILTER_INPUT_WIDTH, GLOBE_ICON, ICON_LENGTH,
        LEFT_COLUMN_WIDTH, MOON_ICON, SCROLL_PADDING, SUN_ICON,
    },
    MainApp, Msg, Page,
};
use crate::{locale::InterfaceSymbol, style_iced};
use iced::{pure, widget::svg::Handle, Alignment, Length, Radio, Space, Svg, Text};
use rab_core::armor_and_skills::Gender;

pub trait MainPage {
    fn get_main_page(&self) -> pure::widget::Row<Msg>;
}

impl MainPage for MainApp {
    fn get_main_page(&self) -> pure::widget::Row<Msg> {
        let mut scrollable_wishes = pure::column()
            .padding(SCROLL_PADDING)
            .spacing(10)
            .align_items(Alignment::Center);
        let size = self.wish_fields.len();
        for (key, wish_field) in self.wish_fields.iter().enumerate() {
            scrollable_wishes = scrollable_wishes.push(get_wishfield_row(
                wish_field,
                &self.filtered_wish_choices,
                size <= 1,
                Msg::RemoveWish(key),
                move |w| Msg::WishSelected(key, w),
                move |value| Msg::SliderChanged(key, value),
            ));
        }

        let filter_text_input = get_skill_filter(&self.value_filter_text_input)
            .width(Length::Units(FILTER_INPUT_WIDTH));

        let row_gender_radio_and_filter = pure::row()
            .spacing(5)
            .push(Radio::new(
                Gender::Female,
                InterfaceSymbol::Female,
                Some(self.selected_gender),
                Msg::GenderChanged,
            ))
            .push(Radio::new(
                Gender::Male,
                InterfaceSymbol::Male,
                Some(self.selected_gender),
                Msg::GenderChanged,
            ))
            .push(Space::with_width(Length::Units(20)))
            .push(filter_text_input);

        let add_wish_button = pure::button(Text::new(InterfaceSymbol::AddWish))
            .style(style_iced::Button::Add)
            .on_press(Msg::AddWish);

        let builds_menu_button = pure::button(Text::new(InterfaceSymbol::ManageBuilds))
            .style(style_iced::Button::Talisman)
            .on_press(Msg::ChangePage(Page::Builds));

        let talisman_button = pure::button(Text::new(InterfaceSymbol::ManageTalismans))
            .style(style_iced::Button::Talisman)
            .on_press(Msg::ChangePage(Page::Talisman));

        let search_button = pure::button(Text::new(InterfaceSymbol::SearchBuilds))
            .style(style_iced::Button::Search)
            .on_press(Msg::Search);

        let col_menu_buttons = pure::column()
            .align_items(Alignment::End)
            .spacing(BUTTON_SPACING)
            .push(builds_menu_button)
            .push(talisman_button);
        let col_other_buttons = pure::column()
            .spacing(BUTTON_SPACING)
            .push(add_wish_button)
            .push(search_button);

        let buttons = pure::row()
            .spacing(BUTTON_SPACING)
            .push(col_other_buttons)
            .push(col_menu_buttons);

        let mut sliders_weapon_slot = pure::row()
            .spacing(5)
            .push(Text::new(InterfaceSymbol::WeaponSlots).width(Length::Units(105)));

        for (index, value) in self.states_values_slider_weapon_slot.iter().enumerate() {
            sliders_weapon_slot = sliders_weapon_slot
                .push(pure::slider(0..=3, *value, move |v| {
                    Msg::WeaponSlotChanged(index, v)
                }))
                .push(Text::new(value.to_string()))
        }

        let column_left = pure::column()
            .spacing(COLUMN_SPACING)
            .push(buttons)
            .push(row_gender_radio_and_filter)
            .push(pure::scrollable(scrollable_wishes).height(Length::FillPortion(2)))
            .push(
                pure::scrollable(armor_desc_to_element(&self.armor_desc))
                    .height(Length::FillPortion(3)),
            )
            .push(Space::with_height(Length::Fill))
            .push(sliders_weapon_slot)
            .align_items(Alignment::Center);

        let column_right = pure::column()
            .spacing(10)
            .push(get_column_builds_found(&self.builds).height(Length::Fill))
            .push(
                pure::row()
                    .height(ICON_LENGTH)
                    .spacing(BUTTON_SPACING)
                    .push(Space::with_width(Length::Fill))
                    .push(
                        update_button(self.update_state, Msg::UpdateArmors)
                            .height(Length::Fill)
                            .width(Length::Shrink),
                    )
                    .push(
                        pure::button(match self.theme {
                            style_iced::Theme::Dark => Svg::new(Handle::from_memory(SUN_ICON)),
                            style_iced::Theme::Light => Svg::new(Handle::from_memory(MOON_ICON)),
                        })
                        .height(ICON_LENGTH)
                        .width(ICON_LENGTH)
                        .on_press(Msg::ToggleTheme),
                    )
                    .push(
                        pure::button(Svg::new(Handle::from_memory(GLOBE_ICON)))
                            .height(ICON_LENGTH)
                            .width(ICON_LENGTH)
                            .on_press(Msg::ChangePage(Page::Lang)),
                    ),
            );
        pure::row()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
    }
}
