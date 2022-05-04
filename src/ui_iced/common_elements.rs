use super::{Msg, UpdateState, WishField};
use crate::locale::InterfaceSymbol;
use crate::{
    locale::{LocalizedArmor, LocalizedSkill},
    style_iced,
};
use iced::{
    alignment, pure,
    widget::svg::{Handle, Svg},
    Alignment, Length, Rule, Space, Text,
};
use rab_core::{
    armor_and_skills::{Armor, Skill},
    build_search::{Build, Jewels},
};
use std::cmp::Reverse;

pub(super) const HEIGHT_BIG_BUTTON: u16 = 60;
pub(super) const BUTTON_SPACING: u16 = 10;
pub(super) const COLUMN_SPACING: u16 = 10;
pub(super) const FILTER_INPUT_WIDTH: u16 = 150;
pub(super) const SCROLL_PADDING: u16 = 20;
pub(super) const LEFT_COLUMN_WIDTH: u16 = 470;
pub(super) const ICON_LENGTH: Length = Length::Units(40);
pub(super) const DETAIL_BUTTON_SIZE: u16 = 20;

pub(super) const CHECK_ICON: &[u8] = include_bytes!("icons/check-solid.svg");
pub(super) const DOWNLOAD_ICON: &[u8] = include_bytes!("icons/cloud-download-alt-solid.svg");
pub(super) const GLOBE_ICON: &[u8] = include_bytes!("icons/globe-europe-solid.svg");
pub(super) const MOON_ICON: &[u8] = include_bytes!("icons/moon-solid.svg");
pub(super) const SUN_ICON: &[u8] = include_bytes!("icons/sun-solid.svg");
pub(super) const SYNC_ICON: &[u8] = include_bytes!("icons/sync-alt-solid.svg");
pub(super) const CROSS_ICON: &[u8] = include_bytes!("icons/times-solid.svg");

pub(super) const HELMET_ICON: &[u8] = include_bytes!("icons/helmet.svg");
pub(super) const CHEST_ICON: &[u8] = include_bytes!("icons/chest.svg");
pub(super) const ARM_ICON: &[u8] = include_bytes!("icons/arm.svg");
pub(super) const WAIST_ICON: &[u8] = include_bytes!("icons/waist.svg");
pub(super) const LEG_ICON: &[u8] = include_bytes!("icons/leg.svg");
pub(super) const TALISMAN_ICON: &[u8] = include_bytes!("icons/talisman.svg");

pub(super) fn get_column_builds_found(builds: &[Build]) -> pure::widget::Column<Msg> {
    let mut builds_column = pure::column()
        .align_items(Alignment::Center)
        .spacing(10)
        .padding(SCROLL_PADDING);
    let size = builds.len();
    if size == 0 {
        builds_column = builds_column.push(Text::new(InterfaceSymbol::NoResult));
    } else {
        for (key, build) in builds.iter().enumerate() {
            let mut details_button = pure::button(
                Text::new("?")
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .style(style_iced::Button::Talisman);
            /*
            if build.weapon_jewels.iter().any(Option::is_some) {
                weapon_button = weapon_button.on_press(Msg::ViewWeaponJewel(build.weapon_jewels));
            }
            */
            details_button = details_button.on_press(Msg::BuildDetails(key));
            let row_build = pure::row()
                .align_items(Alignment::Center)
                .spacing(BUTTON_SPACING)
                .push(details_button.width(Length::Units(DETAIL_BUTTON_SIZE)))
                .push(build_part_to_button(&build.helmet))
                .push(build_part_to_button(&build.chest))
                .push(build_part_to_button(&build.arm))
                .push(build_part_to_button(&build.waist))
                .push(build_part_to_button(&build.leg))
                .push(build_part_to_button(&build.talisman));
            builds_column = builds_column.push(row_build);
            if key < size - 1 {
                builds_column = builds_column.push(Rule::horizontal(1))
            }
        }
    }

    // to center the titles
    let space_width = if SCROLL_PADDING < BUTTON_SPACING {
        0
    } else {
        SCROLL_PADDING - BUTTON_SPACING
    };

    let mut col_titles = pure::row()
        .spacing(BUTTON_SPACING)
        .push(Space::with_width(Length::Units(space_width)))
        .push(Space::with_width(Length::Units(DETAIL_BUTTON_SIZE)));

    for icon in [
        HELMET_ICON,
        CHEST_ICON,
        ARM_ICON,
        WAIST_ICON,
        LEG_ICON,
        TALISMAN_ICON,
    ] {
        col_titles = col_titles.push(
            pure::container(Svg::new(Handle::from_memory(icon)).width(ICON_LENGTH))
                .width(Length::Fill)
                .center_x(),
        );
    }

    pure::column()
        .push(col_titles.push(Space::with_width(Length::Units(space_width))))
        .push(pure::scrollable(builds_column.width(Length::Fill)))
}

pub(super) fn get_wishfield_row<'a>(
    wish_field: &'a WishField,
    skill_list: &'a [LocalizedSkill],
    disable_remove_button: bool,
    on_remove: Msg,
    on_skill_selected: impl Fn(LocalizedSkill) -> Msg + 'static,
    on_slider_changed: impl Fn(u8) -> Msg + 'static,
) -> pure::widget::Row<'a, Msg> {
    let pick_list = pure::pick_list(skill_list, Some(wish_field.selected), on_skill_selected)
        .width(Length::Units(200));
    let mut remove_button =
        pure::button(Text::new(InterfaceSymbol::Remove)).style(style_iced::Button::Remove);
    if !disable_remove_button {
        remove_button = remove_button.on_press(on_remove);
    }
    let slider = pure::slider(
        1..=wish_field.selected.get_limit(),
        wish_field.value_slider,
        on_slider_changed,
    )
    .width(Length::Units(100));
    let text = Text::new(format!("{}", wish_field.value_slider));
    pure::row()
        .spacing(10)
        .push(pick_list)
        .push(slider)
        .push(text)
        .push(remove_button)
}

pub(super) fn get_skill_filter<'a>(value: &str) -> pure::widget::TextInput<'a, Msg> {
    pure::text_input(
        &InterfaceSymbol::SkillFilter.to_string(),
        value,
        Msg::FilterChanged,
    )
    .padding(5)
}

pub(super) fn build_part_to_button<'a>(
    build_part: &Option<(Armor, Jewels)>,
) -> pure::widget::Button<'a, Msg> {
    let button = pure::button(
        pure::container(Text::new(if let Some((armor, _)) = build_part {
            LocalizedArmor(armor).to_string()
        } else {
            InterfaceSymbol::Free.to_string()
        }))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y(),
    )
    .style(style_iced::Button::Result)
    .width(Length::Fill)
    .height(Length::Units(HEIGHT_BIG_BUTTON));
    if build_part.is_none() {
        button
    } else {
        button.on_press(Msg::ArmorDesc(build_part.clone()))
    }
}

pub(super) fn update_button<'a>(
    update_state: UpdateState,
    msg: Msg,
) -> pure::widget::Button<'a, Msg> {
    let b = pure::button(
        pure::row()
            .spacing(BUTTON_SPACING)
            .push(
                Svg::new(Handle::from_memory(match update_state {
                    UpdateState::Initial => DOWNLOAD_ICON,
                    UpdateState::Done => CHECK_ICON,
                    UpdateState::Updating => SYNC_ICON,
                    UpdateState::Problem => CROSS_ICON,
                }))
                .width(ICON_LENGTH),
            )
            .push(
                Text::new(match update_state {
                    UpdateState::Initial => InterfaceSymbol::UpdateArmors,
                    UpdateState::Done => InterfaceSymbol::Updated,
                    UpdateState::Updating => InterfaceSymbol::Updating,
                    UpdateState::Problem => InterfaceSymbol::ProblemCheckConsole,
                })
                .height(Length::Fill)
                .vertical_alignment(alignment::Vertical::Center),
            ),
    );
    match update_state {
        UpdateState::Updating | UpdateState::Done => b,
        _ => b.on_press(msg),
    }
}

pub(super) fn armor_desc_to_element(armor: &Option<(Armor, Jewels)>) -> pure::widget::Column<Msg> {
    if let Some((armor, jewel_skills)) = armor {
        let mut col_armor_stats = pure::column()
            .align_items(Alignment::Center)
            .spacing(5)
            .push(Text::new(LocalizedArmor(armor).to_string()));
        for (style, name, value) in [
            (
                style_iced::Container::Defense,
                InterfaceSymbol::Defense,
                armor.defense as i8,
            ),
            (
                style_iced::Container::Fire,
                InterfaceSymbol::Fire,
                armor.fire,
            ),
            (
                style_iced::Container::Water,
                InterfaceSymbol::Water,
                armor.water,
            ),
            (
                style_iced::Container::Thunder,
                InterfaceSymbol::Thunder,
                armor.thunder,
            ),
            (style_iced::Container::Ice, InterfaceSymbol::Ice, armor.ice),
            (
                style_iced::Container::Dragon,
                InterfaceSymbol::Dragon,
                armor.dragon,
            ),
        ] {
            col_armor_stats = col_armor_stats.push(
                pure::row()
                    .spacing(10)
                    .push(
                        pure::container(Text::new(name))
                            .width(Length::Units(70))
                            .center_x()
                            .style(style),
                    )
                    .push(
                        Text::new(value.to_string())
                            .width(Length::Units(30))
                            .horizontal_alignment(iced::alignment::Horizontal::Right),
                    ),
            )
        }

        if !armor.skills.is_empty() || !armor.slots.is_empty() {
            col_armor_stats = col_armor_stats.push(Space::with_height(Length::Units(10)));
        }

        for (skill, amount) in armor.skills.iter() {
            col_armor_stats = col_armor_stats.push(skill_and_amount(skill, *amount))
        }

        if !armor.skills.is_empty() && !armor.slots.is_empty() {
            col_armor_stats = col_armor_stats.push(Space::with_height(Length::Units(10)));
        }

        let mut slots = armor.slots.clone();
        slots.sort_unstable();

        let mut couple_slot_jewel = Vec::with_capacity(3);

        let mut jewel_skills: Vec<Skill> = jewel_skills.iter().copied().flatten().collect();
        // reverse order
        jewel_skills.sort_unstable_by_key(|a| Reverse(a.get_jewel_size()));

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
                jewel_on_slot(&skill, slot)
            } else {
                pure::container(Text::new(
                    InterfaceSymbol::TemplateFreeSlot
                        .to_string()
                        .replace("{size}", &slot.to_string()),
                ))
                .width(Length::Units(170))
                .center_x()
                .style(style_iced::Container::Ice)
            });
        }

        col_armor_stats
    } else {
        pure::column()
    }
}

pub(super) fn jewel_on_slot<'a>(skill: &Skill, slot: u8) -> pure::widget::Container<'a, Msg> {
    pure::container(Text::new(
        InterfaceSymbol::TemplateJewelOnSlot
            .to_string()
            .replace("{skill}", &LocalizedSkill(*skill).to_string())
            .replace("{size}", &slot.to_string()),
    ))
    .width(Length::Units(170))
    .center_x()
    .style(style_iced::Container::Ice)
}

pub(super) const SKILL_AMOUNT_SIZE: u16 = 150;

pub(super) fn skill_and_amount<'a>(skill: &Skill, amount: u8) -> pure::widget::Container<'a, Msg> {
    pure::container(Text::new(format!("{} x{}", LocalizedSkill(*skill), amount)))
        .width(Length::Units(SKILL_AMOUNT_SIZE))
        .center_x()
        .style(style_iced::Container::Fire)
}
