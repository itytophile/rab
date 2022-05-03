use super::{
    common_elements::{
        get_column_builds_found, get_skill_filter, get_wishfield_row, skill_and_amount,
        BUTTON_SPACING, COLUMN_SPACING, FILTER_INPUT_WIDTH, LEFT_COLUMN_WIDTH, SCROLL_PADDING,
    },
    MainApp, Msg, Page, WishField,
};
use crate::{
    locale::{InterfaceSymbol, LocalizedSkill},
    style_iced,
};
use iced::{pure, Alignment, Length, Space, Text};
use rab_core::armor_and_skills::Armor;

pub trait TalismanPage {
    fn get_talisman_page(&self) -> pure::widget::Row<Msg>;
}

impl TalismanPage for MainApp {
    fn get_talisman_page(&self) -> pure::widget::Row<Msg> {
        let back_button = pure::button(
            pure::container(Text::new(InterfaceSymbol::Back))
                .center_x()
                .width(Length::Units(100)),
        )
        .style(style_iced::Button::Talisman)
        .on_press(Msg::ChangePage(Page::Main));

        let add_talisman_button = pure::button(Text::new(InterfaceSymbol::AddTalisman))
            .style(style_iced::Button::Add)
            .on_press(Msg::AddTalisman);

        let row_buttons = pure::row()
            .spacing(BUTTON_SPACING)
            .push(add_talisman_button)
            .push(back_button);

        let mut talisman_scroll = pure::column()
            .align_items(Alignment::Center)
            .padding(SCROLL_PADDING)
            .spacing(10);

        for (index, talisman) in self.talismans.iter().enumerate() {
            let mut button =
                pure::button(Text::new(&talisman.name)).style(style_iced::Button::Result);
            if !self.is_editing {
                button = button.on_press(Msg::SelectTalisman(Some(index)));
            }
            talisman_scroll = talisman_scroll.push(button);
        }

        let mut column = pure::column()
            .spacing(COLUMN_SPACING)
            .push(row_buttons)
            .push(pure::scrollable(
                talisman_scroll.height(Length::FillPortion(2)),
            ));

        if let Some(index) = &self.selected_talisman {
            let view = if self.is_editing {
                pure::column()
                    .align_items(Alignment::Center)
                    .push(
                        pure::container(get_talisman_editor(
                            &self.states_values_slider_talisman_slot,
                            &self.value_edit_text_input,
                            &self.edit_wish_fields,
                            &self.filtered_wish_choices,
                            &self.value_filter_text_input,
                        ))
                        .padding(10)
                        .style(style_iced::Container::Talisman(self.theme))
                        .max_height(350),
                    )
                    .push(
                        pure::row()
                            .spacing(10)
                            .push(
                                pure::button(Text::new(InterfaceSymbol::RemoveTalisman))
                                    .on_press(Msg::RemoveTalisman)
                                    .style(style_iced::Button::RemoveTalisman),
                            )
                            .push(
                                pure::button(
                                    pure::container(Text::new(InterfaceSymbol::Cancel))
                                        .center_x()
                                        .width(Length::Units(100)),
                                )
                                .style(style_iced::Button::Cancel)
                                .on_press(Msg::CancelEdition),
                            )
                            .push(
                                pure::button(Text::new(InterfaceSymbol::Save))
                                    .style(style_iced::Button::Save)
                                    .on_press(Msg::SaveEdition),
                            ),
                    )
            } else {
                let talisman_desc = talisman_to_element(&self.talismans[*index]);
                pure::column()
                    .align_items(Alignment::Center)
                    .push(
                        pure::container(pure::column().push(talisman_desc).padding(10))
                            .style(style_iced::Container::Talisman(self.theme)),
                    )
                    .push(
                        pure::button(
                            pure::container(Text::new(InterfaceSymbol::Edit))
                                .center_x()
                                .width(Length::Units(100)),
                        )
                        .style(style_iced::Button::Edit)
                        .on_press(Msg::EditTalisman),
                    )
            };

            column = column.push(
                pure::container(view)
                    .center_x()
                    .height(Length::FillPortion(3)),
            );
        } else {
            column = column.push(Space::with_height(Length::FillPortion(3)))
        }

        let mut discard_button = pure::button(Text::new(InterfaceSymbol::DiscardModifications))
            .style(style_iced::Button::Remove);

        let mut save_button =
            pure::button(Text::new(InterfaceSymbol::SaveToFile)).style(style_iced::Button::Add);

        if !self.is_editing {
            discard_button = discard_button.on_press(Msg::DiscardTalismans);
            save_button = save_button.on_press(Msg::SaveTalismans)
        }

        let column_left = column
            .push(
                pure::row()
                    .spacing(10)
                    .push(discard_button)
                    .push(save_button),
            )
            .align_items(Alignment::Center);
        let column_right = get_column_builds_found(&self.builds);
        pure::row()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
    }
}

fn get_talisman_editor<'a>(
    states_values_slider_talisman_slot: &'a [u8],
    value_text_input: &'a str,
    wish_fields: &'a [WishField],
    skill_list: &'a [LocalizedSkill],
    value_filter_text_input: &'a str,
) -> pure::widget::Scrollable<'a, Msg> {
    let text_input = pure::text_input(
        &InterfaceSymbol::TalismanName.to_string(),
        value_text_input,
        Msg::EditTalismanName,
    )
    .padding(5)
    .width(Length::Units(150));

    let filter_text_input =
        get_skill_filter(value_filter_text_input).width(Length::Units(FILTER_INPUT_WIDTH));

    let row = pure::row()
        .spacing(10)
        .push(
            pure::button(Text::new(InterfaceSymbol::AddSkill))
                .on_press(Msg::EditAddSkill)
                .style(style_iced::Button::Add),
        )
        .push(filter_text_input);

    let mut scroll = pure::column()
        .spacing(COLUMN_SPACING)
        .padding(SCROLL_PADDING)
        .align_items(Alignment::Center)
        .push(text_input)
        .push(row);

    for (index, wish_fields) in wish_fields.iter().enumerate() {
        scroll = scroll.push(get_wishfield_row(
            wish_fields,
            skill_list,
            false,
            Msg::EditRemoveSkill(index),
            move |skill| Msg::EditSkillSelected(index, skill),
            move |v| Msg::EditSkillSliderChanged(index, v),
        ));
    }

    let mut sliders_slot = pure::row()
        .spacing(5)
        .push(Text::new(InterfaceSymbol::Slots));
    for (index, value) in states_values_slider_talisman_slot.iter().enumerate() {
        sliders_slot = sliders_slot
            .push(
                pure::slider(0..=3, *value, move |v| Msg::TalismanSlotChanged(index, v))
                    .width(Length::Units(40)),
            )
            .push(Text::new(value.to_string()))
    }
    pure::scrollable(scroll.push(sliders_slot))
}

fn talisman_to_element<'a>(talisman: &Armor) -> pure::widget::Scrollable<'a, Msg> {
    let mut talisman_desc = pure::column()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(&talisman.name));

    for (skill, amount) in talisman.skills.iter() {
        talisman_desc = talisman_desc.push(skill_and_amount(skill, *amount))
    }

    if !talisman.skills.is_empty() && !talisman.slots.is_empty() {
        talisman_desc = talisman_desc.push(Space::with_height(Length::Units(10)))
    }

    for slot in talisman.slots.iter() {
        talisman_desc = talisman_desc.push(
            pure::container(Text::new(
                InterfaceSymbol::TemplateFreeSlot
                    .to_string()
                    .replace("{size}", &slot.to_string()),
            ))
            .width(Length::Units(170))
            .center_x()
            .style(style_iced::Container::Ice),
        )
    }

    pure::scrollable(talisman_desc).height(Length::Units(200))
}
