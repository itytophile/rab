use std::{array, cmp::Ordering};

use crate::build_search::{pre_selection_then_brute_force_search, Build};
use crate::style_iced;
use crate::{
    armor_ron::{get_armor_list, get_talismans, Armor, Gender, Skill},
    build_search::Jewels,
};
use iced::{
    button, pick_list, scrollable, slider, text_input, Align, Button, Column, Container, Element,
    Length, PickList, Radio, Row, Rule, Sandbox, Scrollable, Slider, Space, Text, TextInput,
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

    wish_choices: Vec<Skill>,

    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,

    talismans: Vec<Armor>,

    builds: Vec<Build>,
    states_build_button: Vec<(
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
    )>,

    armor_desc: Option<(Armor, [Option<Skill>; 3])>,

    page: Page,

    selected_gender: Gender,
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
}

const WAISTS_PATH: &str = "armors/waists.ron";
const HELMETS_PATH: &str = "armors/helmets.ron";
const ARMS_PATH: &str = "armors/arms.ron";
const LEGS_PATH: &str = "armors/legs.ron";
const CHESTS_PATH: &str = "armors/chests.ron";

const TALISMANS_PATH: &str = "talismans.ron";

impl Sandbox for MainApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            wish_fields: vec![WishField::default()],

            waists: get_armor_list(WAISTS_PATH),
            helmets: get_armor_list(HELMETS_PATH),
            arms: get_armor_list(ARMS_PATH),
            legs: get_armor_list(LEGS_PATH),
            chests: get_armor_list(CHESTS_PATH),
            talismans: get_talismans(TALISMANS_PATH),

            wish_choices: Skill::ALL.to_vec(),

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
                self.builds = pre_selection_then_brute_force_search(
                    &wishes,
                    &self.helmets,
                    &self.chests,
                    &self.arms,
                    &self.waists,
                    &self.legs,
                    &self.talismans,
                    self.selected_gender,
                );
                self.states_build_button = vec![Default::default(); self.builds.len()];
            }
            Message::ArmorDesc(option) => self.armor_desc = option,
            Message::FilterChanged(text) => {
                self.value_filter_text_input = text;
                self.wish_choices = Skill::ALL
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
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut builds_scrolls = Scrollable::new(&mut self.state_builds_scroll)
            .align_items(Align::Center)
            .spacing(10)
            .padding(20);
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
                let row_build = Row::new()
                    .spacing(10)
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
                    .padding(20)
                    .spacing(10)
                    .align_items(Align::Center);
                let size = self.wish_fields.len();
                for (key, wish_field) in self.wish_fields.iter_mut().enumerate() {
                    let pick_list = PickList::new(
                        &mut wish_field.state_pick_list,
                        &self.wish_choices,
                        Some(wish_field.selected),
                        move |w| Message::WishSelected(key, w),
                    )
                    .width(Length::Units(200));
                    let mut row = Row::new().spacing(10).push(pick_list);
                    let mut remove_button =
                        Button::new(&mut wish_field.state_remove_button, Text::new("Remove"))
                            .style(style_iced::Button::Remove);
                    if size > 1 {
                        remove_button = remove_button.on_press(Message::RemoveWish(key));
                    }
                    let slider = Slider::new(
                        &mut wish_field.state_slider,
                        1..=wish_field.selected.get_limit(),
                        wish_field.value_slider,
                        move |value| Message::SliderChanged(key, value),
                    )
                    .width(Length::Units(100));
                    let text = Text::new(format!("{}", wish_field.value_slider));
                    row = row.push(slider).push(text).push(remove_button);
                    scrollable_wishes = scrollable_wishes.push(row);
                }

                let filter_text_input = TextInput::new(
                    &mut self.state_filter_text_input,
                    "Skill filter",
                    &self.value_filter_text_input,
                    Message::FilterChanged,
                )
                .padding(5)
                .width(Length::Units(150));

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
                    .push(Space::new(Length::Units(20), Length::Shrink))
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
                    .spacing(10)
                    .push(add_wish_button)
                    .push(talisman_button)
                    .push(search_button);
                Column::new()
                    .spacing(10)
                    .push(buttons)
                    .push(row_gender_radio_and_filter)
                    .push(scrollable_wishes.height(Length::FillPortion(2)))
                    .push(
                        Scrollable::new(&mut self.state_desc_scroll)
                            .push(armor_desc_to_element(&self.armor_desc))
                            .align_items(Align::Center)
                            .height(Length::FillPortion(3)),
                    )
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

                let mut talisman_column = Column::new();

                for talisman in self.talismans.iter() {
                    talisman_column = talisman_column.push(Text::new(&talisman.name));
                }

                Column::new().push(back_button).push(talisman_column)
            }
        }
        .align_items(Align::Center);

        let mut col_titles = Row::new();

        for col_name in array::IntoIter::new(["Helmet", "Chest", "Arm", "Waist", "Leg", "Talisman"])
        {
            col_titles = col_titles.push(
                Text::new(col_name)
                    .width(Length::Fill)
                    .horizontal_alignment(iced::HorizontalAlignment::Center),
            );
        }

        let column_right = Column::new()
            .push(col_titles)
            .push(builds_scrolls.width(Length::Fill));

        Row::new()
            .padding(5)
            .push(column_left.width(Length::Units(450)))
            .push(column_right)
            .into()
    }
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
    .height(Length::Units(60));
    if build_part.is_none() {
        button
    } else {
        button.on_press(Message::ArmorDesc(build_part.clone()))
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
