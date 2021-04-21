use crate::armor_ron::{get_armor_list, Armor, Skill};
use crate::build_search::{pre_selection_then_brute_force_search, Build};
use crate::style_iced;
use iced::{
    button, pick_list, scrollable, slider, text_input, Align, Button, Column, Container, Element,
    Length, PickList, Row, Rule, Sandbox, Scrollable, Slider, Space, Text, TextInput,
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
    wish_fields: Vec<WishField>,
    state_add_wish_button: button::State,
    state_search_button: button::State,
    state_filter_text_input: text_input::State,
    value_filter_text_input: String,

    wish_choices: Vec<Skill>,

    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,

    builds: Vec<Build>,
    states_build_button: Vec<(
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
    )>,

    armor_desc: Option<(Armor, [Option<Skill>; 3])>,
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
}

const WAISTS_PATH: &str = "waists.ron";
const HELMETS_PATH: &str = "helmets.ron";
const ARMS_PATH: &str = "arms.ron";
const LEGS_PATH: &str = "legs.ron";
const CHESTS_PATH: &str = "chests.ron";

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
            wish_choices: Skill::ALL.to_vec(),

            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Pick list - Iced")
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
        }
    }

    fn view(&mut self) -> Element<Message> {
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
                    .push(build_part_to_button(&mut state_button.4, &build.leg));
                builds_scrolls = builds_scrolls.push(row_build);
                if key < size - 1 {
                    builds_scrolls = builds_scrolls.push(Rule::horizontal(1))
                }
            }
        }

        let filter_text_input = TextInput::new(
            &mut self.state_filter_text_input,
            "Skill filter",
            &self.value_filter_text_input,
            Message::FilterChanged,
        )
        .padding(5)
        .width(Length::Units(200));

        let add_wish_button = Button::new(&mut self.state_add_wish_button, Text::new("Add wish"))
            .style(style_iced::Button::Add)
            .on_press(Message::AddWish);
        let search_button = Button::new(&mut self.state_search_button, Text::new("Search builds"))
            .style(style_iced::Button::Search)
            .on_press(Message::Search);
        let buttons = Row::new()
            .spacing(10)
            .push(add_wish_button)
            .push(search_button);
        let column_left = Column::new()
            .spacing(10)
            .push(buttons)
            .push(filter_text_input)
            .push(scrollable_wishes.height(Length::FillPortion(2)))
            .push(armor_desc_to_element(&self.armor_desc).height(Length::FillPortion(3)))
            .align_items(Align::Center);

        let mut col_titles = Row::new();

        for col_name in std::array::IntoIter::new(["Helmet", "Chest", "Arm", "Waist", "Leg"]) {
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
            .push(column_left)
            .push(column_right)
            .into()
    }
}

fn build_part_to_button<'a>(
    state: &'a mut button::State,
    build_part: &Option<(Armor, [Option<Skill>; 3])>,
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

fn armor_desc_to_element(armor: &Option<(Armor, [Option<Skill>; 3])>) -> Column<Message> {
    if let Some((armor, skills)) = armor {
        let mut col_armor_stats = Column::new().align_items(Align::Center).spacing(5);
        for (style, name, value) in std::array::IntoIter::new([
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

        if armor.skills.len() > 0 {
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

        let mut has_put_first_jewel = false;

        for skill in skills {
            if let Some(skill) = skill {
                if !has_put_first_jewel {
                    col_armor_stats = col_armor_stats.push(Space::with_height(Length::Units(10)));
                    has_put_first_jewel = true;
                }
                col_armor_stats = col_armor_stats.push(
                    Container::new(Text::new(format!("Jewel {}", skill)))
                        .width(Length::Units(170))
                        .center_x()
                        .style(style_iced::Container::Ice),
                )
            }
        }
        col_armor_stats
    } else {
        Column::new().push(Text::new("None"))
    }
}
