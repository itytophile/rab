use std::{array, cmp::Ordering};

use iced::{
    Align, Button, Column, Container, Element, Length, Radio, Row, Scrollable, Slider, Space, Text,
};

use crate::{
    armor_and_skills::{Armor, Gender, Skill},
    build_search::Jewels,
    style_iced,
};

use super::{MainApp, Message, Page, common_elements::{
        get_column_builds_found, get_skill_filter, get_wishfield_row, BUTTON_SPACING,
        COLUMN_SPACING, FILTER_INPUT_WIDTH, LEFT_COLUMN_WIDTH, SCROLL_PADDING,
    }};

pub trait MainPage {
    fn get_main_page(&mut self) -> Element<Message>;
}

impl MainPage for MainApp {
    fn get_main_page(&mut self) -> Element<Message> {
        let mut scrollable_wishes = Scrollable::new(&mut self.scroll)
            .padding(SCROLL_PADDING)
            .spacing(10)
            .align_items(Align::Center);
        let size = self.wish_fields.len();
        for (key, wish_field) in self.wish_fields.iter_mut().enumerate() {
            scrollable_wishes = scrollable_wishes.push(get_wishfield_row(
                wish_field,
                &self.filtered_wish_choices,
                size <= 1,
                Message::RemoveWish(key),
                move |w| Message::WishSelected(key, w),
                move |value| Message::SliderChanged(key, value),
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
                "Female",
                Some(self.selected_gender),
                Message::GenderChanged,
            ))
            .push(Radio::new(
                Gender::Male,
                "Male",
                Some(self.selected_gender),
                Message::GenderChanged,
            ))
            .push(Space::with_width(Length::Units(20)))
            .push(filter_text_input);

        let add_wish_button = Button::new(&mut self.state_add_wish_button, Text::new("Add wish"))
            .style(style_iced::Button::Add)
            .on_press(Message::AddWish);
        let talisman_button = Button::new(
            &mut self.state_talisman_button,
            Text::new("Manage talismans"),
        )
        .style(style_iced::Button::Talisman)
        .on_press(Message::ChangePage(Page::Talisman));
        let search_button = Button::new(&mut self.state_search_button, Text::new("Search builds"))
            .style(style_iced::Button::Search)
            .on_press(Message::Search);
        let buttons = Row::new()
            .spacing(BUTTON_SPACING)
            .push(add_wish_button)
            .push(talisman_button)
            .push(search_button);

        let mut weapon_jewels_row = Row::new()
            .spacing(5)
            .push(Space::with_width(Length::Units(105)));

        for jewel in self.selected_weapon_jewels.iter() {
            if let Some(jewel) = jewel {
                weapon_jewels_row = weapon_jewels_row.push(
                    Container::new(Text::new(jewel.to_string()))
                        .center_x()
                        .style(style_iced::Container::Ice)
                        .width(Length::Fill),
                )
            } else {
                weapon_jewels_row = weapon_jewels_row.push(Space::with_width(Length::Fill))
            }
        }

        let mut sliders_weapon_slot = Row::new()
            .spacing(5)
            .push(Text::new("Weapon slots").width(Length::Units(105)));
        for (index, (state, value)) in self.states_values_slider_weapon_slot.iter_mut().enumerate()
        {
            sliders_weapon_slot = sliders_weapon_slot
                .push(Slider::new(state, 0..=3, *value, move |v| {
                    Message::WeaponSlotChanged(index, v)
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
                    .align_items(Align::Center)
                    .height(Length::FillPortion(3)),
            )
            .push(Space::with_height(Length::Fill))
            .push(weapon_jewels_row)
            .push(sliders_weapon_slot)
            .align_items(Align::Center);
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
                    .push(Space::with_width(Length::Fill))
                    .push(Button::new(
                        &mut self.state_settings_button,
                        Text::new("Settings"),
                    ).on_press(Message::ChangePage(Page::Settings))),
            );
        Row::new()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
            .into()
    }
}

fn armor_desc_to_element(armor: &Option<(Armor, Jewels)>) -> Column<Message> {
    if let Some((armor, jewel_skills)) = armor {
        let mut col_armor_stats = Column::new()
            .align_items(Align::Center)
            .spacing(5)
            .push(Text::new(&armor.name));
        for (style, name, value) in array::IntoIter::new([
            (
                style_iced::Container::Defense,
                "Defense",
                armor.defense as i8,
            ),
            (style_iced::Container::Fire, "Fire", armor.fire),
            (style_iced::Container::Water, "Water", armor.water),
            (style_iced::Container::Thunder, "Thunder", armor.thunder),
            (style_iced::Container::Ice, "Ice", armor.ice),
            (style_iced::Container::Dragon, "Dragon", armor.dragon),
        ]) {
            col_armor_stats = col_armor_stats.push(
                Row::new()
                    .spacing(10)
                    .push(
                        Container::new(Text::new(name))
                            .width(Length::Units(70))
                            .center_x()
                            .style(style),
                    )
                    .push(
                        Text::new(value.to_string())
                            .width(Length::Units(30))
                            .horizontal_alignment(iced::HorizontalAlignment::Right),
                    ),
            )
        }

        if armor.skills.len() > 0 || armor.slots.len() > 0 {
            col_armor_stats = col_armor_stats.push(Space::with_height(Length::Units(10)));
        }

        for (skill, amount) in armor.skills.iter() {
            col_armor_stats = col_armor_stats.push(
                Container::new(Text::new(format!("{} x{}", skill, amount)))
                    .width(Length::Units(150))
                    .center_x()
                    .style(style_iced::Container::Fire),
            )
        }

        if armor.skills.len() > 0 && armor.slots.len() > 0 {
            col_armor_stats = col_armor_stats.push(Space::with_height(Length::Units(10)));
        }

        let mut slots = armor.slots.clone();
        slots.sort_unstable();

        let mut couple_slot_jewel = Vec::with_capacity(3);

        let mut jewel_skills: Vec<Skill> = jewel_skills
            .iter()
            .copied()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        // reverse order
        jewel_skills.sort_unstable_by(|a, b| {
            if a.get_jewel_size() > b.get_jewel_size() {
                Ordering::Less
            } else if a.get_jewel_size() < b.get_jewel_size() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        // to be sure that the jewel will be on the most little slot possible
        let mut to_remove = None;
        'slot_loop: for slot in slots {
            if let Some(index) = to_remove {
                jewel_skills.swap_remove(index);
                to_remove = None;
            }
            for (index, skill) in jewel_skills.iter().enumerate() {
                if skill.get_jewel_size().unwrap() <= slot {
                    couple_slot_jewel.push((slot, Some(*skill)));
                    to_remove = Some(index);
                    continue 'slot_loop;
                }
            }
            couple_slot_jewel.push((slot, None));
        }

        for (slot, skill) in couple_slot_jewel {
            col_armor_stats = col_armor_stats.push(if let Some(skill) = skill {
                Container::new(Text::new(format!("{} on lvl {} slot", skill, slot)))
                    .width(Length::Units(170))
                    .center_x()
                    .style(style_iced::Container::Ice)
            } else {
                Container::new(Text::new(format!("Free lvl {} slot", slot)))
                    .width(Length::Units(170))
                    .center_x()
                    .style(style_iced::Container::Ice)
            });
        }

        col_armor_stats
    } else {
        Column::new()
    }
}
