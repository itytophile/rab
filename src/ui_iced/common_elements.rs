use std::array;

use iced::{
    button, scrollable, text_input,
    widget::svg::{Handle, Svg},
    Align, Button, Column, Container, HorizontalAlignment, Length, PickList, Row, Rule, Scrollable,
    Slider, Space, Text, TextInput, VerticalAlignment,
};

use crate::{
    armor_and_skills::{Armor, Skill},
    build_search::{Build, Jewels},
    style_iced,
};

use super::{Msg, UpdateState, WishField};

use crate::locale::InterfaceSymbol;

pub(super) const HEIGHT_BIG_BUTTON: u16 = 60;
pub(super) const BUTTON_SPACING: u16 = 10;
pub(super) const COLUMN_SPACING: u16 = 10;
pub(super) const FILTER_INPUT_WIDTH: u16 = 150;
pub(super) const SCROLL_PADDING: u16 = 20;
pub(super) const LEFT_COLUMN_WIDTH: u16 = 470;

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
            let mut weapon_button = Button::new(
                &mut state_button.6,
                Text::new("?")
                    .vertical_alignment(VerticalAlignment::Center)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .style(style_iced::Button::Talisman);
            if build.weapon_jewels.iter().any(Option::is_some) {
                weapon_button = weapon_button.on_press(Msg::ViewWeaponJewel(build.weapon_jewels));
            }
            let row_build = Row::new()
                .align_items(Align::Center)
                .spacing(BUTTON_SPACING)
                .push(weapon_button.width(Length::Units(20)))
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

    let mut col_titles = Row::new()
        .spacing(BUTTON_SPACING)
        .push(Space::with_width(Length::Units(20)));

    for col_name in array::IntoIter::new([
        InterfaceSymbol::Helmet,
        InterfaceSymbol::Chest,
        InterfaceSymbol::Arm,
        InterfaceSymbol::Waist,
        InterfaceSymbol::Leg,
        InterfaceSymbol::Talisman,
    ]) {
        col_titles = col_titles.push(
            Text::new(col_name)
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        );
    }

    Column::new()
        .push(col_titles)
        .push(builds_scrolls.width(Length::Fill))
}

pub(super) fn get_wishfield_row<'a>(
    wish_field: &'a mut WishField,
    skill_list: &'a [Skill],
    disable_remove_button: bool,
    on_remove: Msg,
    on_skill_selected: impl Fn(Skill) -> Msg + 'static,
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

fn build_part_to_button<'a>(
    state: &'a mut button::State,
    build_part: &Option<(Armor, Jewels)>,
) -> Button<'a, Msg> {
    let button = Button::new(
        state,
        Container::new(Text::new(if let Some((armor, _)) = build_part {
            armor.to_string()
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
                UpdateState::Initial => {
                    include_bytes!("icons/cloud-download-alt-solid.svg").to_vec()
                }
                UpdateState::Done => include_bytes!("icons/check-solid.svg").to_vec(),
                UpdateState::Updating => include_bytes!("icons/sync-alt-solid.svg").to_vec(),
                UpdateState::Problem => include_bytes!("icons/times-solid.svg").to_vec(),
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
