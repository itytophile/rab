mod armor_ron;
mod build_search;

use armor_ron::{get_armor_list, Armor, Skill, SKILL_LIMIT_JEWEL_SIZE};
use build_search::Build;
use iced::{
    button, pick_list, scrollable, slider, Align, Button, Column, Container, Element, Length,
    PickList, Row, Rule, Sandbox, Scrollable, Settings, Slider, Text,
};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

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
struct Example {
    scroll: scrollable::State,
    state_builds_scroll: scrollable::State,
    wish_fields: Vec<WishField>,
    state_add_wish_button: button::State,
    state_search_button: button::State,

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
enum Message {
    WishSelected(usize, Skill),
    AddWish,
    RemoveWish(usize),
    SliderChanged(usize, u8),
    Search,
    ArmorDesc(Option<(Armor, [Option<Skill>; 3])>),
}

const WAISTS_PATH: &str = "waists.ron";
const HELMETS_PATH: &str = "helmets.ron";
const ARMS_PATH: &str = "arms.ron";
const LEGS_PATH: &str = "legs.ron";
const CHESTS_PATH: &str = "chests.ron";

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            wish_fields: vec![WishField::default()],
            waists: get_armor_list(WAISTS_PATH),
            helmets: get_armor_list(HELMETS_PATH),
            arms: get_armor_list(ARMS_PATH),
            legs: get_armor_list(LEGS_PATH),
            chests: get_armor_list(CHESTS_PATH),

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
                self.builds = build_search::pre_selection_then_brute_force_search(
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
                &Skill::ALL[..],
                Some(wish_field.selected),
                move |w| Message::WishSelected(key, w),
            );
            let mut row = Row::new().spacing(10).push(pick_list);
            let mut remove_button =
                Button::new(&mut wish_field.state_remove_button, Text::new("Remove"))
                    .style(style::Button::Remove);
            if size > 1 {
                remove_button = remove_button.on_press(Message::RemoveWish(key));
            }
            let slider = Slider::new(
                &mut wish_field.state_slider,
                1..=SKILL_LIMIT_JEWEL_SIZE
                    .get(&wish_field.selected)
                    .unwrap()
                    .limit,
                wish_field.value_slider,
                move |value| Message::SliderChanged(key, value),
            )
            .width(Length::Units(100));
            let text = Text::new(format!("{}", wish_field.value_slider));
            row = row.push(slider).push(text).push(remove_button);
            scrollable_wishes = scrollable_wishes.push(row);
        }

        let mut builds_scrolls = Scrollable::new(&mut self.state_builds_scroll)
            .width(Length::Fill)
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
        let add_wish_button = Button::new(&mut self.state_add_wish_button, Text::new("Add wish"))
            .style(style::Button::Add)
            .on_press(Message::AddWish);
        let search_button = Button::new(&mut self.state_search_button, Text::new("Search builds"))
            .style(style::Button::Search)
            .on_press(Message::Search);
        let buttons = Row::new()
            .spacing(10)
            .push(add_wish_button)
            .push(search_button);
        let column_right = Column::new()
            .spacing(10)
            .push(buttons)
            .push(scrollable_wishes)
            .align_items(Align::Center);

        let row = Row::new().push(column_right).push(builds_scrolls);

        let content = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(row.height(Length::Fill))
            .push(
                armor_desc_to_element(&self.armor_desc)
                    .height(Length::Fill)
                    .width(Length::Fill),
            );

        Container::new(content)
            .padding(30)
            .width(Length::Fill)
            .center_x()
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
    .style(style::Button::Result)
    .width(Length::Fill)
    .height(Length::Units(60));
    if build_part.is_none() {
        button
    } else {
        button.on_press(Message::ArmorDesc(build_part.clone()))
    }
}

fn armor_desc_to_element(armor: &Option<(Armor, [Option<Skill>; 3])>) -> Row<Message> {
    if let Some((armor, skills)) = armor {
        let mut col_armor_stats = Column::new()
            .push(Text::new(&armor.name))
            .push(Text::new(&armor.defense.to_string()))
            .push(Text::new(&armor.fire.to_string()))
            .push(Text::new(&armor.water.to_string()))
            .push(Text::new(&armor.thunder.to_string()))
            .push(Text::new(&armor.ice.to_string()))
            .push(Text::new(&armor.dragon.to_string()));
        for (skill, amount) in armor.skills.iter() {
            col_armor_stats = col_armor_stats.push(Text::new(format!("{} x{}", skill, amount)))
        }
        let mut col_skills = Column::new();
        for skill in skills {
            col_skills = if let Some(skill) = skill {
                col_skills.push(Text::new(skill.to_string()).height(Length::Fill))
            } else {
                col_skills.push(Text::new("None").height(Length::Fill))
            }
        }
        Row::new()
            .push(col_armor_stats.width(Length::Fill))
            .push(col_skills.width(Length::Fill))
    } else {
        Row::new().push(Text::new("None"))
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Filter { selected: bool },
        Icon,
        Remove,
        Add,
        Search,
        Result,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Filter { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                            border_radius: 10.0,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
                Button::Icon => button::Style {
                    text_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..button::Style::default()
                },
                Button::Remove => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.27, 0.41))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Add => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.28, 0.78, 0.56))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Search => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.28, 0.37, 0.78))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Result => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.94, 0.96, 0.98))),
                    border_radius: 5.0,
                    text_color: Color::from_rgb(0.16, 0.44, 0.66),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
                    _ => active.text_color,
                },
                background: match self {
                    Button::Result => Some(Background::Color(Color::from_rgb(0.89, 0.94, 0.98))),
                    _ => active.background,
                },
                shadow_offset: match self {
                    Button::Result => active.shadow_offset,
                    _ => active.shadow_offset + Vector::new(0.0, 1.0),
                },
                ..active
            }
        }
    }
}

/*
use armor_ron::{get_armor_list, Armor, Skill};
use build_search::Jewels;

const WAISTS_PATH: &str = "waists.ron";
const HELMETS_PATH: &str = "helmets.ron";
const ARMS_PATH: &str = "arms.ron";
const LEGS_PATH: &str = "legs.ron";
const CHESTS_PATH: &str = "chests.ron";

fn main() {

    let waists: Vec<Armor> = get_armor_list(WAISTS_PATH);
    let helmets: Vec<Armor> = get_armor_list(HELMETS_PATH);
    let arms: Vec<Armor> = get_armor_list(ARMS_PATH);
    let legs: Vec<Armor> = get_armor_list(LEGS_PATH);
    let chests: Vec<Armor> = get_armor_list(CHESTS_PATH);

    dbg!(waists.len());
    dbg!(helmets.len());
    dbg!(arms.len());
    dbg!(legs.len());
    dbg!(chests.len());

    let wishes = &[
        (Skill::Earplugs, 4),
        (Skill::CriticalBoost, 3),
        (Skill::TremorResistance, 3),
    ];

    let builds = build_search::pre_selection_then_brute_force_search(wishes, helmets, chests, arms, waists, legs);

    for build in &builds {
        println!(
            "{}\n{}\n{}\n{}\n{}\n",
            debug_build_part(&build.helmet),
            debug_build_part(&build.chest),
            debug_build_part(&build.arm),
            debug_build_part(&build.waist),
            debug_build_part(&build.leg)
        )
    }

}



*/
