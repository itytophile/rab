use super::{
    common_elements::{
        armor_desc_to_element, skill_and_amount, ARM_ICON, BUTTON_SPACING, CHEST_ICON,
        COLUMN_SPACING, HELMET_ICON, ICON_LENGTH, LEG_ICON, SCROLL_PADDING, SKILL_AMOUNT_SIZE,
        TALISMAN_ICON, WAIST_ICON,
    },
    MainApp, Msg, Page,
};
use crate::{
    locale::{InterfaceSymbol, LocalizedSkill},
    style_iced,
};
use iced::{
    pure,
    widget::svg::{Handle, Svg},
    Alignment, Length, Space, Text,
};

// need refactoring

pub trait DetailsPage {
    fn get_details_page(&self, on_save_builds: bool) -> pure::widget::Column<Msg>;
}

impl DetailsPage for MainApp {
    fn get_details_page(&self, on_save_builds: bool) -> pure::widget::Column<Msg> {
        let mut row = pure::row();
        let mut row_title = pure::row()
            .push(Space::with_width(Length::Units(SCROLL_PADDING)))
            .push(Space::with_width(Length::Units(SKILL_AMOUNT_SIZE)));
        for icon in [
            HELMET_ICON,
            CHEST_ICON,
            ARM_ICON,
            WAIST_ICON,
            LEG_ICON,
            TALISMAN_ICON,
        ] {
            row_title = row_title.push(
                pure::container(Svg::new(Handle::from_memory(icon)).width(ICON_LENGTH))
                    .width(Length::Fill)
                    .center_x(),
            );
        }
        let build_index = self.details_build_index;
        let build = self.focused_build.as_ref().unwrap();

        let mut col_skills = pure::column().spacing(5);

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
            row = row.push(if part.is_none() {
                pure::Element::<Msg>::from(
                    pure::container(Text::new(InterfaceSymbol::Free))
                        .width(Length::Fill)
                        .center_x(),
                )
            } else {
                armor_desc_to_element(part).width(Length::Fill).into()
            })
        }

        let mut weapon_jewels_row = pure::row()
            .push(Text::new(InterfaceSymbol::WeaponSlots))
            .spacing(5);

        for jewel in build.weapon_jewels.iter().flatten() {
            weapon_jewels_row = weapon_jewels_row.push(
                pure::container(Text::new(LocalizedSkill(*jewel).to_string()))
                    .center_x()
                    .style(style_iced::Container::Ice)
                    .width(Length::Units(170)),
            )
        }

        pure::column()
            .spacing(COLUMN_SPACING)
            .padding(5)
            .push(Space::with_width(Length::Units(5)))
            .push(
                pure::container(
                    pure::row()
                        .spacing(BUTTON_SPACING)
                        .push(
                            pure::text_input(
                                &InterfaceSymbol::NewBuildName.to_string(),
                                &self.value_edit_text_input,
                                Msg::EditTalismanName,
                            )
                            .padding(5)
                            .width(Length::Units(200)),
                        )
                        .push(
                            pure::button(Text::new(InterfaceSymbol::SaveBuild))
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
                pure::scrollable(
                    pure::column()
                        .padding(SCROLL_PADDING)
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(row)
                        .push(
                            pure::container(weapon_jewels_row)
                                .width(Length::Fill)
                                .center_x(),
                        ),
                )
                .height(Length::Fill),
            )
            .push(pure::row().push(Space::with_width(Length::Fill)).push(
                pure::button(Text::new(InterfaceSymbol::Back)).on_press(if on_save_builds {
                    Msg::ChangePage(Page::Builds)
                } else {
                    Msg::ChangePage(Page::Main)
                }),
            ))
    }
}
