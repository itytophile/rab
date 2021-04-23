use std::{array, cmp::Ordering};

use crate::style_iced;
use crate::{
    armor_ron::save_talismans_to_file,
    build_search::{pre_selection_then_brute_force_search, Build},
};
use crate::{
    armor_ron::{get_armor_list, get_talismans, Armor, Gender, Skill},
    build_search::Jewels,
};
use iced::{
    button, pick_list, scrollable, slider, text_input, Align, Button, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Radio, Row, Rule, Sandbox, Scrollable, Slider, Space,
    Text, TextInput, VerticalAlignment,
};

struct WishField {
    state_pick_list: pick_list::State<Skill>,
    selected: Skill,
    state_remove_button: button::State,
    state_slider: slider::State,
    value_slider: u8,
}

impl Default for WishField {
    fn default() -> Self {
        Self {
            value_slider: 1,
            state_pick_list: Default::default(),
            selected: Default::default(),
            state_remove_button: Default::default(),
            state_slider: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct MainApp {
    scroll: scrollable::State,
    state_builds_scroll: scrollable::State,
    state_desc_scroll: scrollable::State,
    wish_fields: Vec<WishField>,

    state_add_wish_button: button::State,
    state_talisman_button: button::State,
    state_search_button: button::State,

    state_filter_text_input: text_input::State,
    value_filter_text_input: String,

    filtered_wish_choices: Vec<Skill>,

    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,

    talismans: Vec<Armor>,
    states_talisman_button: Vec<button::State>,

    builds: Vec<Build>,
    states_build_button: Vec<(
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
        button::State, //talisman
        button::State, //weapon
    )>,

    armor_desc: Option<(Armor, [Option<Skill>; 3])>,

    page: Page,

    selected_gender: Gender,

    states_values_slider_weapon_slot: [(slider::State, u8); 3],

    selected_weapon_jewels: Jewels,

    state_talisman_scroll: scrollable::State,
    selected_talisman: Option<usize>,
    state_talisman_desc_scroll: scrollable::State,

    state_edit_button: button::State,

    is_editing: bool,

    states_values_slider_talisman_slot: [(slider::State, u8); 3],
    state_edit_text_input: text_input::State,
    value_edit_text_input: String,

    edit_wish_fields: Vec<WishField>,
    state_edit_add_skill_button: button::State,

    state_cancel_button: button::State,
    state_remove_talisman_button: button::State,

    state_save_talismans_button: button::State,
    state_discard_talismans_button: button::State,
}

enum Page {
    Main,
    Talisman,
}

impl Default for Page {
    fn default() -> Self {
        Page::Main
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    WishSelected(usize, Skill),
    AddWish,
    RemoveWish(usize),
    SliderChanged(usize, u8),
    Search,
    ArmorDesc(Option<(Armor, [Option<Skill>; 3])>),
    FilterChanged(String),
    ToggleTalisman,
    GenderChanged(Gender),
    WeaponSlotChanged(usize, u8),
    ViewWeaponJewel(Jewels),
    SelectTalisman(Option<usize>),
    EditTalisman,
    SaveEdition,
    CancelEdition,
    TalismanSlotChanged(usize, u8),
    EditTalismanName(String),
    EditSkillSelected(usize, Skill),
    EditAddSkill,
    EditRemoveSkill(usize),
    EditSkillSliderChanged(usize, u8),
    RemoveTalisman,
    AddTalisman,
    SaveTalismans,
    DiscardTalismans,
}

const WAISTS_PATH: &str = "armors/waists.ron";
const HELMETS_PATH: &str = "armors/helmets.ron";
const ARMS_PATH: &str = "armors/arms.ron";
const LEGS_PATH: &str = "armors/legs.ron";
const CHESTS_PATH: &str = "armors/chests.ron";

const TALISMANS_PATH: &str = "talismans.ron";

const HEIGHT_BIG_BUTTON: u16 = 60;
const BUTTON_SPACING: u16 = 10;
const COLUMN_SPACING: u16 = 10;
const FILTER_INPUT_WIDTH: u16 = 150;
const SCROLL_PADDING: u16 = 20;
const LEFT_COLUMN_WIDTH: u16 = 470;

impl MainApp {
    fn clear_talisman_editor(&mut self) {
        for (_, slider_value) in self.states_values_slider_talisman_slot.iter_mut() {
            *slider_value = 0
        }
        self.edit_wish_fields.clear();
    }
}

impl Sandbox for MainApp {
    type Message = Message;

    fn new() -> Self {
        let talismans = match get_talismans(TALISMANS_PATH) {
            Ok(talismans) => {
                println!("Talisman file succesfully loaded.");
                talismans
            }
            Err(err) => {
                println!(
                    "Can't read the talisman file: {}\nEmpty talisman list loaded.",
                    err
                );
                vec![]
            }
        };
        let states_talisman_button = vec![Default::default(); talismans.len()];
        Self {
            wish_fields: vec![WishField::default()],

            waists: get_armor_list(WAISTS_PATH),
            helmets: get_armor_list(HELMETS_PATH),
            arms: get_armor_list(ARMS_PATH),
            legs: get_armor_list(LEGS_PATH),
            chests: get_armor_list(CHESTS_PATH),
            talismans,
            states_talisman_button,

            filtered_wish_choices: Skill::ALL.to_vec(),

            selected_gender: Gender::Female,

            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("RAB - Rusty Armor Builds")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WishSelected(key, wish) => {
                self.wish_fields[key].selected = wish;
                self.wish_fields[key].value_slider = 1
            }
            Message::AddWish => self.wish_fields.push(WishField::default()),
            Message::RemoveWish(index) => {
                self.wish_fields.remove(index);
            }
            Message::SliderChanged(index, value) => self.wish_fields[index].value_slider = value,
            Message::Search => {
                let wishes: Vec<(Skill, u8)> = self
                    .wish_fields
                    .iter()
                    .map(|wish| (wish.selected, wish.value_slider))
                    .collect();
                self.selected_weapon_jewels = Default::default();
                self.builds = pre_selection_then_brute_force_search(
                    &wishes,
                    &self.helmets,
                    &self.chests,
                    &self.arms,
                    &self.waists,
                    &self.legs,
                    &self.talismans,
                    self.selected_gender,
                    [
                        self.states_values_slider_weapon_slot[0].1,
                        self.states_values_slider_weapon_slot[1].1,
                        self.states_values_slider_weapon_slot[2].1,
                    ],
                );
                self.states_build_button = vec![Default::default(); self.builds.len()];
            }
            Message::ArmorDesc(option) => self.armor_desc = option,
            Message::FilterChanged(text) => {
                self.value_filter_text_input = text;
                self.filtered_wish_choices = Skill::ALL
                    .iter()
                    .copied()
                    .filter(|skill| {
                        skill
                            .to_string()
                            .to_ascii_lowercase()
                            .contains(&self.value_filter_text_input.to_ascii_lowercase())
                    })
                    .collect();
            }
            Message::ToggleTalisman => match self.page {
                Page::Main => self.page = Page::Talisman,
                _ => self.page = Page::Main,
            },
            Message::GenderChanged(gender) => self.selected_gender = gender,
            Message::WeaponSlotChanged(index, value) => {
                self.states_values_slider_weapon_slot[index].1 = value
            }

            Message::ViewWeaponJewel(jewels) => self.selected_weapon_jewels = jewels,
            Message::SelectTalisman(index) => self.selected_talisman = index,
            Message::EditTalisman => {
                self.is_editing = true;
                let talisman = &self.talismans[self.selected_talisman.unwrap()];
                // We provide the actual talisman's data to the edit form
                self.value_edit_text_input = talisman.name.clone();

                for &(skill, amount) in talisman.skills.iter() {
                    self.edit_wish_fields.push(WishField {
                        selected: skill,
                        value_slider: amount,
                        ..Default::default()
                    })
                }

                for (slot, (_, slider_value)) in talisman
                    .slots
                    .iter()
                    .zip(self.states_values_slider_talisman_slot.iter_mut())
                {
                    *slider_value = *slot;
                }
            }
            Message::SaveEdition => {
                self.is_editing = false;
                let talisman = &mut self.talismans[self.selected_talisman.unwrap()];
                talisman.name = self.value_edit_text_input.clone();
                talisman.skills = self
                    .edit_wish_fields
                    .iter()
                    .map(|field| (field.selected, field.value_slider))
                    .collect();
                talisman.slots = self
                    .states_values_slider_talisman_slot
                    .iter()
                    .map(|(_, slot)| *slot)
                    .filter(|slot| *slot > 0)
                    .collect();
                self.clear_talisman_editor()
            }
            Message::CancelEdition => {
                self.is_editing = false;
                self.clear_talisman_editor()
            }
            Message::TalismanSlotChanged(index, value) => {
                self.states_values_slider_talisman_slot[index].1 = value
            }
            Message::EditTalismanName(name) => self.value_edit_text_input = name,
            Message::EditSkillSelected(key, wish) => {
                self.edit_wish_fields[key].selected = wish;
                self.edit_wish_fields[key].value_slider = 1
            }
            Message::EditAddSkill => self.edit_wish_fields.push(WishField::default()),
            Message::EditRemoveSkill(index) => {
                self.edit_wish_fields.remove(index);
            }
            Message::EditSkillSliderChanged(index, value) => {
                self.edit_wish_fields[index].value_slider = value
            }
            Message::RemoveTalisman => {
                let index = self.selected_talisman.unwrap();
                self.talismans.remove(index);
                self.states_talisman_button.remove(index);
                self.clear_talisman_editor();
                self.is_editing = false;
                self.selected_talisman = None;
            }
            Message::AddTalisman => {
                self.talismans.push(Armor {
                    name: "New talisman".to_string(),
                    skills: vec![(Skill::Botanist, 1)],
                    ..Default::default()
                });
                self.states_talisman_button.push(Default::default())
            }
            Message::SaveTalismans => {
                match save_talismans_to_file(&self.talismans, TALISMANS_PATH) {
                    Ok(path) => println!("Talismans saved to {}", path),
                    Err(err) => println!("Unable to save the talismans: {}", err),
                }
            }
            Message::DiscardTalismans => {
                self.selected_talisman = None;
                let talismans = match get_talismans(TALISMANS_PATH) {
                    Ok(talismans) => {
                        println!("Talisman file succesfully loaded.");
                        talismans
                    }
                    Err(err) => {
                        println!(
                            "Can't read the talisman file: {}\nEmpty talisman list loaded.",
                            err
                        );
                        vec![]
                    }
                };
                let states_talisman_button = vec![Default::default(); talismans.len()];

                self.talismans = talismans;
                self.states_talisman_button = states_talisman_button;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut builds_scrolls = Scrollable::new(&mut self.state_builds_scroll)
            .align_items(Align::Center)
            .spacing(10)
            .padding(SCROLL_PADDING);
        let size = self.builds.len();
        if size == 0 {
            builds_scrolls = builds_scrolls.push(Text::new("No Result"));
        } else {
            for ((key, build), state_button) in self
                .builds
                .iter()
                .enumerate()
                .zip(self.states_build_button.iter_mut())
            {
                let mut weapon_button = Button::new(
                    &mut state_button.6,
                    Text::new("?")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .style(style_iced::Button::Talisman);
                if build.weapon_jewels.iter().any(Option::is_some) {
                    weapon_button =
                        weapon_button.on_press(Message::ViewWeaponJewel(build.weapon_jewels));
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

        let column_left = match self.page {
            Page::Main => {
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

                let add_wish_button =
                    Button::new(&mut self.state_add_wish_button, Text::new("Add wish"))
                        .style(style_iced::Button::Add)
                        .on_press(Message::AddWish);
                let talisman_button = Button::new(
                    &mut self.state_talisman_button,
                    Text::new("Manage talismans"),
                )
                .style(style_iced::Button::Talisman)
                .on_press(Message::ToggleTalisman);
                let search_button =
                    Button::new(&mut self.state_search_button, Text::new("Search builds"))
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
                for (index, (state, value)) in
                    self.states_values_slider_weapon_slot.iter_mut().enumerate()
                {
                    sliders_weapon_slot = sliders_weapon_slot
                        .push(Slider::new(state, 0..=3, *value, move |v| {
                            Message::WeaponSlotChanged(index, v)
                        }))
                        .push(Text::new(value.to_string()))
                }

                Column::new()
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
            }
            Page::Talisman => {
                let back_button = Button::new(
                    &mut self.state_talisman_button,
                    Container::new(Text::new("Back"))
                        .center_x()
                        .width(Length::Units(100)),
                )
                .style(style_iced::Button::Talisman)
                .on_press(Message::ToggleTalisman);

                let add_talisman_button =
                    Button::new(&mut self.state_add_wish_button, Text::new("Add talisman"))
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
                                .style(style_iced::Container::Talisman)
                                .max_height(350),
                            )
                            .push(
                                Row::new()
                                    .spacing(10)
                                    .push(
                                        Button::new(
                                            &mut self.state_remove_talisman_button,
                                            Text::new("Remove"),
                                        )
                                        .on_press(Message::RemoveTalisman)
                                        .style(style_iced::Button::RemoveTalisman),
                                    )
                                    .push(
                                        Button::new(
                                            &mut self.state_cancel_button, // cheating
                                            Container::new(Text::new("Cancel"))
                                                .center_x()
                                                .width(Length::Units(100)),
                                        )
                                        .style(style_iced::Button::Cancel)
                                        .on_press(Message::CancelEdition),
                                    )
                                    .push(
                                        Button::new(
                                            &mut self.state_edit_button, // cheating
                                            Text::new("Save"),
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
                                    .style(style_iced::Container::Talisman),
                            )
                            .push(
                                Button::new(
                                    &mut self.state_edit_button,
                                    Container::new(Text::new("Edit"))
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
                    Text::new("Discard modifications"),
                )
                .style(style_iced::Button::Remove);

                let mut save_button = Button::new(
                    &mut self.state_save_talismans_button,
                    Text::new("Save to file"),
                )
                .style(style_iced::Button::Add);

                if !self.is_editing {
                    discard_button = discard_button.on_press(Message::DiscardTalismans);
                    save_button = save_button.on_press(Message::SaveTalismans)
                }

                column.push(
                    Row::new()
                        .spacing(10)
                        .push(discard_button)
                        .push(save_button),
                )
            }
        }
        .align_items(Align::Center);

        let mut col_titles = Row::new()
            .spacing(BUTTON_SPACING)
            .push(Space::with_width(Length::Units(20)));

        for col_name in array::IntoIter::new(["Helmet", "Chest", "Arm", "Waist", "Leg", "Talisman"])
        {
            col_titles = col_titles.push(
                Text::new(col_name)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            );
        }

        let column_right = Column::new()
            .push(col_titles)
            .push(builds_scrolls.width(Length::Fill));

        Row::new()
            .padding(5)
            .push(column_left.width(Length::Units(LEFT_COLUMN_WIDTH)))
            .push(column_right)
            .into()
    }
}

fn get_wishfield_row<'a>(
    wish_field: &'a mut WishField,
    skill_list: &'a [Skill],
    disable_remove_button: bool,
    on_remove: Message,
    on_skill_selected: impl Fn(Skill) -> Message + 'static,
    on_slider_changed: impl Fn(u8) -> Message + 'static,
) -> Row<'a, Message> {
    let pick_list = PickList::new(
        &mut wish_field.state_pick_list,
        skill_list,
        Some(wish_field.selected),
        on_skill_selected,
    )
    .width(Length::Units(200));
    let mut remove_button = Button::new(&mut wish_field.state_remove_button, Text::new("Remove"))
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

fn get_skill_filter<'a>(state: &'a mut text_input::State, value: &str) -> TextInput<'a, Message> {
    TextInput::new(state, "Skill filter", value, Message::FilterChanged).padding(5)
}

fn build_part_to_button<'a>(
    state: &'a mut button::State,
    build_part: &Option<(Armor, Jewels)>,
) -> Button<'a, Message> {
    let button = Button::new(
        state,
        Container::new(Text::new(if let Some((armor, _)) = build_part {
            &armor.name
        } else {
            "Free"
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
        button.on_press(Message::ArmorDesc(build_part.clone()))
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
        "New talisman name",
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
            Button::new(state_add_button, Text::new("Add skill"))
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

    let mut sliders_slot = Row::new().spacing(5).push(Text::new("Slots"));
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
            Container::new(Text::new(format!("Free lvl {} slot", slot)))
                .width(Length::Units(170))
                .center_x()
                .style(style_iced::Container::Ice),
        )
    }

    talisman_desc
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
