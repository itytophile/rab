use iced::{
    widget::svg::Handle, Alignment, Button, Column, Element, Length, Radio, Row, Scrollable,
    Slider, Space, Svg, Text,
};

use crate::{locale::InterfaceSymbol, style_iced};

use rab_core::armor_and_skills::Gender;

use super::{
    common_elements::{
        armor_desc_to_element, get_column_builds_found, get_skill_filter, get_wishfield_row,
        update_button, BUTTON_SPACING, COLUMN_SPACING, FILTER_INPUT_WIDTH, GLOBE_ICON, ICON_SIZE,
        LEFT_COLUMN_WIDTH, MOON_ICON, SCROLL_PADDING, SUN_ICON,
    },
    MainApp, Msg, Page,
};

pub trait MainPage {
    fn get_main_page(&mut self) -> Element<Msg>;
}

impl MainPage for MainApp {
    fn get_main_page(&mut self) -> Element<Msg> {
        let mut scrollable_wishes = Scrollable::new(&mut self.scroll)
            .padding(SCROLL_PADDING)
            .spacing(10)
            .align_items(Alignment::Center);
        let size = self.wish_fields.len();
        for (key, wish_field) in self.wish_fields.iter_mut().enumerate() {
            scrollable_wishes = scrollable_wishes.push(get_wishfield_row(
                wish_field,
                &self.filtered_wish_choices,
                size <= 1,
                Msg::RemoveWish(key),
                move |w| Msg::WishSelected(key, w),
                move |value| Msg::SliderChanged(key, value),
            ));
        }

        let filter_text_input = get_skill_filter(
            &mut self.state_filter_text_input,
            &self.value_filter_text_input,
        )
        .width(Length::Units(FILTER_INPUT_WIDTH));

        let row_gender_radio_and_filter = Row::new()
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

        let add_wish_button = Button::new(
            &mut self.state_add_wish_button,
            Text::new(InterfaceSymbol::AddWish),
        )
        .style(style_iced::Button::Add)
        .on_press(Msg::AddWish);

        let builds_menu_button = Button::new(
            &mut self.state_builds_menu_button,
            Text::new(InterfaceSymbol::ManageBuilds),
        )
        .style(style_iced::Button::Talisman)
        .on_press(Msg::ChangePage(Page::Builds));

        let talisman_button = Button::new(
            &mut self.state_talisman_button,
            Text::new(InterfaceSymbol::ManageTalismans),
        )
        .style(style_iced::Button::Talisman)
        .on_press(Msg::ChangePage(Page::Talisman));

        let search_button = Button::new(
            &mut self.state_search_button,
            Text::new(InterfaceSymbol::SearchBuilds),
        )
        .style(style_iced::Button::Search)
        .on_press(Msg::Search);

        let col_menu_buttons = Column::new()
            .align_items(Alignment::End)
            .spacing(BUTTON_SPACING)
            .push(builds_menu_button)
            .push(talisman_button);
        let col_other_buttons = Column::new()
            .spacing(BUTTON_SPACING)
            .push(add_wish_button)
            .push(search_button);

        let buttons = Row::new()
            .spacing(BUTTON_SPACING)
            .push(col_other_buttons)
            .push(col_menu_buttons);

        let mut sliders_weapon_slot = Row::new()
            .spacing(5)
            .push(Text::new(InterfaceSymbol::WeaponSlots).width(Length::Units(105)));
        for (index, (state, value)) in self.states_values_slider_weapon_slot.iter_mut().enumerate()
        {
            sliders_weapon_slot = sliders_weapon_slot
                .push(Slider::new(state, 0..=3, *value, move |v| {
                    Msg::WeaponSlotChanged(index, v)
                }))
                .push(Text::new(value.to_string()))
        }

        let column_left = Column::new()
            .spacing(COLUMN_SPACING)
            .push(buttons)
            .push(row_gender_radio_and_filter)
            .push(scrollable_wishes.height(Length::FillPortion(2)))
            .push(
                Scrollable::new(&mut self.state_desc_scroll)
                    .push(armor_desc_to_element(&self.armor_desc))
                    .align_items(Alignment::Center)
                    .height(Length::FillPortion(3)),
            )
            .push(Space::with_height(Length::Fill))
            .push(sliders_weapon_slot)
            .align_items(Alignment::Center);

        let column_right = Column::new()
            .spacing(10)
            .push(
                get_column_builds_found(
                    &mut self.state_builds_scroll,
                    &self.builds,
                    &mut self.states_build_button,
                )
                .height(Length::Fill),
            )
            .push(
                Row::new()
                    .height(Length::Units(ICON_SIZE))
                    .spacing(BUTTON_SPACING)
                    .push(Space::with_width(Length::Fill))
                    .push(
                        update_button(
                            &mut self.state_update_button,
                            self.update_state,
                            Msg::UpdateArmors,
                        )
                        .height(Length::Fill),
                    )
                    .push(
                        Button::new(
                            &mut self.state_theme_button,
                            match self.theme {
                                style_iced::Theme::Dark => Svg::new(Handle::from_memory(
                                    // I'm scared of the to_vec(), maybe I need
                                    // to create the vectors beforehand but
                                    // I'm just praying the compiler to optimize it.
                                    SUN_ICON.to_vec(),
                                )),
                                style_iced::Theme::Light => {
                                    Svg::new(Handle::from_memory(MOON_ICON.to_vec()))
                                }
                            },
                        )
                        .height(Length::Fill)
                        .on_press(Msg::ToggleTheme),
                    )
                    .push(
                        Button::new(
                            &mut self.state_lang_button,
                            Svg::new(Handle::from_memory(GLOBE_ICON.to_vec())),
                        )
                        .height(Length::Fill)
                        .on_press(Msg::ChangePage(Page::Lang)),
                    ),
            );
        Row::new()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
            .into()
    }
}
