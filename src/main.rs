mod armor_ron;
mod build_search;

use armor_ron::{get_armor_list, Armor, Skill, SKILL_LIMIT_JEWEL_SIZE};
use build_search::Jewels;
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
    wish_fields: Vec<WishField>,
    state_add_wish_button: button::State,
    state_search_button: button::State,

    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,
}

#[derive(Debug, Clone)]
enum Message {
    WishSelected(usize, Skill),
    AddWish,
    RemoveWish(usize),
    SliderChanged(usize, u8),
    Search,
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
                let builds = build_search::pre_selection_then_brute_force_search(
                    &wishes,
                    &self.helmets,
                    &self.chests,
                    &self.arms,
                    &self.waists,
                    &self.legs,
                );
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
        }
    }

    fn view(&mut self) -> Element<Message> {
        let add_wish_button = Button::new(&mut self.state_add_wish_button, Text::new("Add wish"))
            .on_press(Message::AddWish);
        let search_button = Button::new(&mut self.state_search_button, Text::new("Search builds"))
            .on_press(Message::Search);
        let buttons = Row::new()
            .spacing(10)
            .push(add_wish_button)
            .push(search_button);
        let mut column = Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(buttons);

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
                Button::new(&mut wish_field.state_remove_button, Text::new("Remove"));
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
            column = column.push(row);
        }

        let row = Row::new().spacing(10).push(column);

        let content = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(row);

        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
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
fn debug_build_part(part: &Option<(Armor, Jewels)>) -> String {
    match part {
        None => "None".to_string(),
        Some((armor, jewels)) => format!("{}:{:?}", armor.name, jewels),
    }
}