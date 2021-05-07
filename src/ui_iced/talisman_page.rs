use iced::{
    button, scrollable, slider, text_input, Align, Button, Column, Container, Element, Length, Row,
    Scrollable, Slider, Space, Text, TextInput,
};

use crate::{
    armor_and_skills::{Armor, Skill},
    locale::InterfaceSymbol,
    style_iced,
};

use super::{
    common_elements::{
        get_column_builds_found, get_skill_filter, get_wishfield_row, BUTTON_SPACING,
        COLUMN_SPACING, FILTER_INPUT_WIDTH, LEFT_COLUMN_WIDTH, SCROLL_PADDING,
    },
    MainApp, Message, Page, WishField,
};

pub trait TalismanPage {
    fn get_talisman_page(&mut self) -> Element<Message>;
}

impl TalismanPage for MainApp {
    fn get_talisman_page(&mut self) -> Element<Message> {
        let back_button = Button::new(
            &mut self.state_talisman_button,
            Container::new(Text::new(InterfaceSymbol::Back))
                .center_x()
                .width(Length::Units(100)),
        )
        .style(style_iced::Button::Talisman)
        .on_press(Message::ChangePage(Page::Main));

        let add_talisman_button = Button::new(
            &mut self.state_add_wish_button,
            Text::new(InterfaceSymbol::AddTalisman),
        )
        .style(style_iced::Button::Add)
        .on_press(Message::AddTalisman);

        let row_buttons = Row::new()
            .spacing(BUTTON_SPACING)
            .push(add_talisman_button)
            .push(back_button);

        let mut talisman_scroll = Scrollable::new(&mut self.state_talisman_scroll)
            .align_items(Align::Center)
            .padding(SCROLL_PADDING)
            .spacing(10);

        for (index, (talisman, state_button)) in self
            .talismans
            .iter()
            .zip(self.states_talisman_button.iter_mut())
            .enumerate()
        {
            let mut button = Button::new(state_button, Text::new(&talisman.name))
                .style(style_iced::Button::Result);
            if !self.is_editing {
                button = button.on_press(Message::SelectTalisman(Some(index)));
            }
            talisman_scroll = talisman_scroll.push(button);
        }

        let mut column = Column::new()
            .spacing(COLUMN_SPACING)
            .push(row_buttons)
            .push(talisman_scroll.height(Length::FillPortion(2)));

        if let Some(index) = &self.selected_talisman {
            let view = if self.is_editing {
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Container::new(get_talisman_editor(
                            &mut self.state_talisman_desc_scroll,
                            &mut self.states_values_slider_talisman_slot,
                            &mut self.state_edit_text_input,
                            &self.value_edit_text_input,
                            &mut self.edit_wish_fields,
                            &self.filtered_wish_choices,
                            &mut self.state_filter_text_input,
                            &self.value_filter_text_input,
                            &mut self.state_edit_add_skill_button,
                        ))
                        .padding(10)
                        .style(style_iced::Container::Talisman(self.theme))
                        .max_height(350),
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Button::new(
                                    &mut self.state_remove_talisman_button,
                                    Text::new(InterfaceSymbol::RemoveTalisman),
                                )
                                .on_press(Message::RemoveTalisman)
                                .style(style_iced::Button::RemoveTalisman),
                            )
                            .push(
                                Button::new(
                                    &mut self.state_cancel_button, // cheating
                                    Container::new(Text::new(InterfaceSymbol::Cancel))
                                        .center_x()
                                        .width(Length::Units(100)),
                                )
                                .style(style_iced::Button::Cancel)
                                .on_press(Message::CancelEdition),
                            )
                            .push(
                                Button::new(
                                    &mut self.state_edit_button, // cheating
                                    Text::new(InterfaceSymbol::Save),
                                )
                                .style(style_iced::Button::Save)
                                .on_press(Message::SaveEdition),
                            ),
                    )
            } else {
                let talisman_desc = talisman_to_element(
                    &self.talismans[*index],
                    &mut self.state_talisman_desc_scroll,
                );
                Column::new()
                    .align_items(Align::Center)
                    .push(
                        Container::new(talisman_desc)
                            .padding(10)
                            .style(style_iced::Container::Talisman(self.theme)),
                    )
                    .push(
                        Button::new(
                            &mut self.state_edit_button,
                            Container::new(Text::new(InterfaceSymbol::Edit))
                                .center_x()
                                .width(Length::Units(100)),
                        )
                        .style(style_iced::Button::Edit)
                        .on_press(Message::EditTalisman),
                    )
            };

            column = column.push(
                Container::new(view)
                    .center_x()
                    .height(Length::FillPortion(3)),
            );
        } else {
            column = column.push(Space::with_height(Length::FillPortion(3)))
        }

        let mut discard_button = Button::new(
            &mut self.state_discard_talismans_button,
            Text::new(InterfaceSymbol::DiscardModifications),
        )
        .style(style_iced::Button::Remove);

        let mut save_button = Button::new(
            &mut self.state_save_talismans_button,
            Text::new(InterfaceSymbol::SaveToFile),
        )
        .style(style_iced::Button::Add);

        if !self.is_editing {
            discard_button = discard_button.on_press(Message::DiscardTalismans);
            save_button = save_button.on_press(Message::SaveTalismans)
        }

        let column_left = column
            .push(
                Row::new()
                    .spacing(10)
                    .push(discard_button)
                    .push(save_button),
            )
            .align_items(Align::Center);
        let column_right = get_column_builds_found(
            &mut self.state_builds_scroll,
            &self.builds,
            &mut self.states_build_button,
        );
        Row::new()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
            .into()
    }
}

fn get_talisman_editor<'a>(
    state_scroll: &'a mut scrollable::State,
    states_values_slider_talisman_slot: &'a mut [(slider::State, u8)],
    state_text_input: &'a mut text_input::State,
    value_text_input: &str,
    wish_fields: &'a mut [WishField],
    skill_list: &'a [Skill],
    state_filter_text_input: &'a mut text_input::State,
    value_filter_text_input: &'a str,
    state_add_button: &'a mut button::State,
) -> Scrollable<'a, Message> {
    let text_input = TextInput::new(
        state_text_input,
        &InterfaceSymbol::TalismanName.to_string(),
        value_text_input,
        Message::EditTalismanName,
    )
    .padding(5)
    .width(Length::Units(150));

    let filter_text_input = get_skill_filter(state_filter_text_input, value_filter_text_input)
        .width(Length::Units(FILTER_INPUT_WIDTH));

    let row = Row::new()
        .spacing(10)
        .push(
            Button::new(state_add_button, Text::new(InterfaceSymbol::AddSkill))
                .on_press(Message::EditAddSkill)
                .style(style_iced::Button::Add),
        )
        .push(filter_text_input);

    let mut scroll = Scrollable::new(state_scroll)
        .spacing(COLUMN_SPACING)
        .padding(SCROLL_PADDING)
        .align_items(Align::Center)
        .push(text_input)
        .push(row);

    for (index, wish_fields) in wish_fields.iter_mut().enumerate() {
        scroll = scroll.push(get_wishfield_row(
            wish_fields,
            skill_list,
            false,
            Message::EditRemoveSkill(index),
            move |skill| Message::EditSkillSelected(index, skill),
            move |v| Message::EditSkillSliderChanged(index, v),
        ));
    }

    let mut sliders_slot = Row::new()
        .spacing(5)
        .push(Text::new(InterfaceSymbol::Slots));
    for (index, (state, value)) in states_values_slider_talisman_slot.iter_mut().enumerate() {
        sliders_slot = sliders_slot
            .push(
                Slider::new(state, 0..=3, *value, move |v| {
                    Message::TalismanSlotChanged(index, v)
                })
                .width(Length::Units(40)),
            )
            .push(Text::new(value.to_string()))
    }
    scroll.push(sliders_slot)
}

fn talisman_to_element<'a>(
    talisman: &Armor,
    state_scroll: &'a mut scrollable::State,
) -> Scrollable<'a, Message> {
    let mut talisman_desc = Scrollable::new(state_scroll)
        .max_height(200)
        .align_items(Align::Center)
        .spacing(5)
        .push(Text::new(&talisman.name));

    for (skill, amount) in talisman.skills.iter() {
        talisman_desc = talisman_desc.push(
            Container::new(Text::new(format!("{} x{}", skill, amount)))
                .width(Length::Units(150))
                .center_x()
                .style(style_iced::Container::Fire),
        )
    }

    if talisman.skills.len() > 0 && talisman.slots.len() > 0 {
        talisman_desc = talisman_desc.push(Space::with_height(Length::Units(10)))
    }

    for slot in talisman.slots.iter() {
        talisman_desc = talisman_desc.push(
            Container::new(Text::new(
                InterfaceSymbol::TemplateFreeSlot
                    .to_string()
                    .replace("{size}", &slot.to_string()),
            ))
            .width(Length::Units(170))
            .center_x()
            .style(style_iced::Container::Ice),
        )
    }

    talisman_desc
}
