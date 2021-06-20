use iced::{
    widget::svg::{Handle, Svg},
    Align, Button, Column, Container, Element, Length, Row, Scrollable, Space, Text, TextInput,
};

use crate::{
    locale::{InterfaceSymbol, LocalizedSkill},
    style_iced,
};

use super::{
    common_elements::{
        armor_desc_to_element, skill_and_amount, ARM_ICON, BUTTON_SPACING, CHEST_ICON,
        COLUMN_SPACING, HELMET_ICON, ICON_SIZE, LEG_ICON, SCROLL_PADDING, SKILL_AMOUNT_SIZE,
        TALISMAN_ICON, WAIST_ICON,
    },
    MainApp, Msg, Page,
};

// need refactoring

pub trait DetailsPage {
    fn get_details_page(&mut self, on_save_builds: bool) -> Element<Msg>;
}

impl DetailsPage for MainApp {
    fn get_details_page(&mut self, on_save_builds: bool) -> Element<Msg> {
        let mut row = Row::new();
        let mut row_title = Row::new()
            .push(Space::with_width(Length::Units(SCROLL_PADDING)))
            .push(Space::with_width(Length::Units(SKILL_AMOUNT_SIZE)));
        for icon in [
            HELMET_ICON.to_vec(),
            CHEST_ICON.to_vec(),
            ARM_ICON.to_vec(),
            WAIST_ICON.to_vec(),
            LEG_ICON.to_vec(),
            TALISMAN_ICON.to_vec(),
        ] {
            row_title = row_title.push(
                Container::new(Svg::new(Handle::from_memory(icon)).width(Length::Units(ICON_SIZE)))
                    .width(Length::Fill)
                    .center_x(),
            );
        }
        let build_index = self.details_build_index;
        let build = self.focused_build.as_ref().unwrap();

        let mut col_skills = Column::new().spacing(5);

        for (skill, amount) in self.total_skills_and_amounts_focused_build.iter().rev() {
            col_skills = col_skills.push(skill_and_amount(skill, *amount));
        }

        row = row.push(col_skills);

        for part in [
            &build.helmet,
            &build.chest,
            &build.arm,
            &build.waist,
            &build.leg,
            &build.talisman,
        ] {
            row = row.push::<Element<Msg>>(if part.is_none() {
                Container::new(Text::new(InterfaceSymbol::Free))
                    .width(Length::Fill)
                    .center_x()
                    .into()
            } else {
                armor_desc_to_element(part).width(Length::Fill).into()
            })
        }

        let mut weapon_jewels_row = Row::new()
            .push(Text::new(InterfaceSymbol::WeaponSlots))
            .spacing(5);

        for jewel in build.weapon_jewels.iter().flatten() {
            weapon_jewels_row = weapon_jewels_row.push(
                Container::new(Text::new(LocalizedSkill(*jewel).to_string()))
                    .center_x()
                    .style(style_iced::Container::Ice)
                    .width(Length::Units(170)),
            )
        }

        Column::new()
            .spacing(COLUMN_SPACING)
            .padding(5)
            .push(Space::with_width(Length::Units(5)))
            .push(
                Container::new(
                    Row::new()
                        .spacing(BUTTON_SPACING)
                        .push(
                            TextInput::new(
                                &mut self.state_edit_text_input,
                                &InterfaceSymbol::NewBuildName.to_string(),
                                &self.value_edit_text_input,
                                Msg::EditTalismanName,
                            )
                            .padding(5)
                            .width(Length::Units(200)),
                        )
                        .push(
                            Button::new(
                                &mut self.state_save_talismans_button,
                                Text::new(InterfaceSymbol::SaveBuild),
                            )
                            .style(style_iced::Button::Add)
                            .on_press(if on_save_builds {
                                Msg::EditSavedBuild(self.details_build_name.clone())
                            } else {
                                Msg::SaveBuild(build_index)
                            }),
                        ),
                )
                .width(Length::Fill)
                .center_x(),
            )
            .push(row_title.push(Space::with_width(Length::Units(SCROLL_PADDING))))
            .push(
                Scrollable::new(&mut self.scroll)
                    .padding(SCROLL_PADDING)
                    .height(Length::Fill)
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(row)
                    .push(
                        Container::new(weapon_jewels_row)
                            .width(Length::Fill)
                            .center_x(),
                    ),
            )
            .push(
                Row::new().push(Space::with_width(Length::Fill)).push(
                    Button::new(
                        &mut self.state_lang_button,
                        Text::new(InterfaceSymbol::Back),
                    )
                    .on_press(if on_save_builds {
                        Msg::ChangePage(Page::Builds)
                    } else {
                        Msg::ChangePage(Page::Main)
                    }),
                ),
            )
            .into()
    }
}
