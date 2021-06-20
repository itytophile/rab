use std::{array, cmp::Ordering};

use iced::{
    button, scrollable, text_input,
    widget::svg::{Handle, Svg},
    Align, Button, Column, Container, HorizontalAlignment, Length, PickList, Row, Rule, Scrollable,
    Slider, Space, Text, TextInput, VerticalAlignment,
};

use crate::{locale::{LocalizedArmor, LocalizedSkill}, style_iced};

use rab_core::{
    armor_and_skills::{Armor, Skill},
    build_search::{Build, Jewels},
};

use super::{Msg, UpdateState, WishField};

use crate::locale::InterfaceSymbol;

pub(super) const HEIGHT_BIG_BUTTON: u16 = 60;
pub(super) const BUTTON_SPACING: u16 = 10;
pub(super) const COLUMN_SPACING: u16 = 10;
pub(super) const FILTER_INPUT_WIDTH: u16 = 150;
pub(super) const SCROLL_PADDING: u16 = 20;
pub(super) const LEFT_COLUMN_WIDTH: u16 = 470;
pub(super) const ICON_SIZE: u16 = 40;
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

pub(super) fn get_column_builds_found<'a>(
    state_builds_scroll: &'a mut scrollable::State,
    builds: &'a [Build],
    states_build_button: &'a mut [(
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
        button::State, // talisman
        button::State, // weapon
    )],
) -> Column<'a, Msg> {
    let mut builds_scrolls = Scrollable::new(state_builds_scroll)
        .align_items(Align::Center)
        .spacing(10)
        .padding(SCROLL_PADDING);
    let size = builds.len();
    if size == 0 {
        builds_scrolls = builds_scrolls.push(Text::new(InterfaceSymbol::NoResult));
    } else {
        for ((key, build), state_button) in builds
            .iter()
            .enumerate()
            .zip(states_build_button.iter_mut())
        {
            let mut details_button = Button::new(
                &mut state_button.6,
                Text::new("?")
                    .vertical_alignment(VerticalAlignment::Center)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .style(style_iced::Button::Talisman);
            /*
            if build.weapon_jewels.iter().any(Option::is_some) {
                weapon_button = weapon_button.on_press(Msg::ViewWeaponJewel(build.weapon_jewels));
            }
            */
            details_button = details_button.on_press(Msg::BuildDetails(key));
            let row_build = Row::new()
                .align_items(Align::Center)
                .spacing(BUTTON_SPACING)
                .push(details_button.width(Length::Units(DETAIL_BUTTON_SIZE)))
                .push(build_part_to_button(&mut state_button.0, &build.helmet))
                .push(build_part_to_button(&mut state_button.1, &build.chest))
                .push(build_part_to_button(&mut state_button.2, &build.arm))
                .push(build_part_to_button(&mut state_button.3, &build.waist))
                .push(build_part_to_button(&mut state_button.4, &build.leg))
                .push(build_part_to_button(&mut state_button.5, &build.talisman));
            builds_scrolls = builds_scrolls.push(row_build);
            if key < size - 1 {
                builds_scrolls = builds_scrolls.push(Rule::horizontal(1))
            }
        }
    }

    // to center the titles
    let space_width = if SCROLL_PADDING < BUTTON_SPACING {
        0
    } else {
        SCROLL_PADDING - BUTTON_SPACING
    };

    let mut col_titles = Row::new()
        .spacing(BUTTON_SPACING)
        .push(Space::with_width(Length::Units(space_width)))
        .push(Space::with_width(Length::Units(DETAIL_BUTTON_SIZE)));

    for icon in array::IntoIter::new([
        HELMET_ICON.to_vec(),
        CHEST_ICON.to_vec(),
        ARM_ICON.to_vec(),
        WAIST_ICON.to_vec(),
        LEG_ICON.to_vec(),
        TALISMAN_ICON.to_vec(),
    ]) {
        col_titles = col_titles.push(
            Container::new(Svg::new(Handle::from_memory(icon)).width(Length::Units(ICON_SIZE)))
                .width(Length::Fill)
                .center_x(),
        );
    }

    Column::new()
        .push(col_titles.push(Space::with_width(Length::Units(space_width))))
        .push(builds_scrolls.width(Length::Fill))
}

pub(super) fn get_wishfield_row<'a>(
    wish_field: &'a mut WishField,
    skill_list: &'a [LocalizedSkill],
    disable_remove_button: bool,
    on_remove: Msg,
    on_skill_selected: impl Fn(LocalizedSkill) -> Msg + 'static,
    on_slider_changed: impl Fn(u8) -> Msg + 'static,
) -> Row<'a, Msg> {
    let pick_list = PickList::new(
        &mut wish_field.state_pick_list,
        skill_list,
        Some(wish_field.selected),
        on_skill_selected,
    )
    .width(Length::Units(200));
    let mut remove_button = Button::new(
        &mut wish_field.state_remove_button,
        Text::new(InterfaceSymbol::Remove),
    )
    .style(style_iced::Button::Remove);
    if !disable_remove_button {
        remove_button = remove_button.on_press(on_remove);
    }
    let slider = Slider::new(
        &mut wish_field.state_slider,
        1..=wish_field.selected.get_limit(),
        wish_field.value_slider,
        on_slider_changed,
    )
    .width(Length::Units(100));
    let text = Text::new(format!("{}", wish_field.value_slider));
    Row::new()
        .spacing(10)
        .push(pick_list)
        .push(slider)
        .push(text)
        .push(remove_button)
}

pub(super) fn get_skill_filter<'a>(
    state: &'a mut text_input::State,
    value: &str,
) -> TextInput<'a, Msg> {
    TextInput::new(
        state,
        &InterfaceSymbol::SkillFilter.to_string(),
        value,
        Msg::FilterChanged,
    )
    .padding(5)
}

pub(super) fn build_part_to_button<'a>(
    state: &'a mut button::State,
    build_part: &Option<(Armor, Jewels)>,
) -> Button<'a, Msg> {
    let button = Button::new(
        state,
        Container::new(Text::new(if let Some((armor, _)) = build_part {
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
    state: &'a mut button::State,
    update_state: UpdateState,
    msg: Msg,
) -> Button<'a, Msg> {
    let b = Button::new(
        state,
        Row::new()
            .spacing(BUTTON_SPACING)
            .height(Length::Fill)
            .push(Svg::new(Handle::from_memory(match update_state {
                UpdateState::Initial => DOWNLOAD_ICON.to_vec(),
                UpdateState::Done => CHECK_ICON.to_vec(),
                UpdateState::Updating => SYNC_ICON.to_vec(),
                UpdateState::Problem => CROSS_ICON.to_vec(),
            })))
            .push(
                Text::new(match update_state {
                    UpdateState::Initial => InterfaceSymbol::UpdateArmors,
                    UpdateState::Done => InterfaceSymbol::Updated,
                    UpdateState::Updating => InterfaceSymbol::Updating,
                    UpdateState::Problem => InterfaceSymbol::ProblemCheckConsole,
                })
                .height(Length::Fill)
                .vertical_alignment(VerticalAlignment::Center),
            ),
    );
    match update_state {
        UpdateState::Updating | UpdateState::Done => b,
        _ => b.on_press(msg),
    }
}

pub(super) fn armor_desc_to_element(armor: &Option<(Armor, Jewels)>) -> Column<Msg> {
    if let Some((armor, jewel_skills)) = armor {
        let mut col_armor_stats = Column::new()
            .align_items(Align::Center)
            .spacing(5)
            .push(Text::new(LocalizedArmor(armor).to_string()));
        for (style, name, value) in array::IntoIter::new([
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
            col_armor_stats = col_armor_stats.push(skill_and_amount(skill, *amount))
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
                jewel_on_slot(&skill, slot)
            } else {
                Container::new(Text::new(
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
        Column::new()
    }
}

pub(super) fn jewel_on_slot<'a>(skill: &Skill, slot: u8) -> Container<'a, Msg> {
    Container::new(Text::new(
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

pub(super) fn skill_and_amount<'a>(skill: &Skill, amount: u8) -> Container<'a, Msg> {
    Container::new(Text::new(format!("{} x{}", LocalizedSkill(*skill), amount)))
        .width(Length::Units(SKILL_AMOUNT_SIZE))
        .center_x()
        .style(style_iced::Container::Fire)
}
