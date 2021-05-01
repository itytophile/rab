mod common_elements;
mod error_page;
mod main_page;
mod settings_page;
mod talisman_page;

use std::collections::HashMap;

use crate::{
    armor_and_skills::save_talismans_to_file,
    build_search::{pre_selection_then_brute_force_search, Build},
    locale::{get_locales, Locale},
};
use crate::{
    armor_and_skills::{get_armor_list, get_talismans, Armor, Gender, Skill},
    build_search::Jewels,
};
use iced::{button, pick_list, scrollable, slider, text_input, Element, Sandbox};

use main_page::MainPage;
use talisman_page::TalismanPage;

use self::{error_page::get_error_page, settings_page::SettingsPage};

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

    state_settings_button: button::State,

    locales: HashMap<String, Locale>,
    selected_locale: String,

    state_buttons_locale: Vec<button::State>,
}

#[derive(Debug, Clone)]
pub enum RabError {
    ArmorFiles,
}

#[derive(Debug, Clone)]
pub enum Page {
    Main,
    Talisman,
    Settings,
    Err(String, RabError),
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
    ChangePage(Page),
    LocaleChanged(String),
}

const WAISTS_PATH: &str = "armors/waists.ron";
const HELMETS_PATH: &str = "armors/helmets.ron";
const ARMS_PATH: &str = "armors/arms.ron";
const LEGS_PATH: &str = "armors/legs.ron";
const CHESTS_PATH: &str = "armors/chests.ron";

const TALISMANS_PATH: &str = "talismans.ron";

const LOCALE_DIR_PATH: &str = "locale";

impl MainApp {
    fn clear_talisman_editor(&mut self) {
        for (_, slider_value) in self.states_values_slider_talisman_slot.iter_mut() {
            *slider_value = 0
        }
        self.edit_wish_fields.clear();
    }
}
fn get_all_armors_from_file(
) -> Result<(Vec<Armor>, Vec<Armor>, Vec<Armor>, Vec<Armor>, Vec<Armor>), ron::Error> {
    Ok((
        get_armor_list(HELMETS_PATH)?,
        get_armor_list(CHESTS_PATH)?,
        get_armor_list(ARMS_PATH)?,
        get_armor_list(WAISTS_PATH)?,
        get_armor_list(LEGS_PATH)?,
    ))
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
        let mut page = Page::Main;
        let (helmets, chests, arms, waists, legs) = match get_all_armors_from_file() {
            Ok(lists) => lists,
            Err(err) => {
                page = Page::Err(err.to_string(), RabError::ArmorFiles);
                (vec![], vec![], vec![], vec![], vec![])
            }
        };

        let locales = match get_locales(LOCALE_DIR_PATH) {
            Ok(locales) => locales,
            Err(err) => {
                println!(
                    "Error with localization files at {}\n{}",
                    LOCALE_DIR_PATH, err
                );
                HashMap::with_capacity(0)
            }
        };

        let selected_locale = "English".to_string();

        *super::LOCALE.lock().unwrap() = locales.get(&selected_locale).cloned();

        let state_buttons_locale = vec![Default::default(); locales.len()];

        Self {
            wish_fields: vec![WishField::default()],

            waists,
            helmets,
            arms,
            legs,
            chests,
            talismans,
            states_talisman_button,

            filtered_wish_choices: Skill::ALL.to_vec(),

            selected_gender: Gender::Female,

            page,

            locales,
            selected_locale,

            state_buttons_locale,

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
            Message::ChangePage(page) => self.page = page,
            Message::LocaleChanged(new_locale) => {
                *super::LOCALE.lock().unwrap() = self.locales.get(&new_locale).cloned();
                self.selected_locale = new_locale;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &self.page {
            Page::Main => self.get_main_page(),
            Page::Talisman => self.get_talisman_page(),
            // I don't know if this is possible to give this function
            // just a &str. The compiler complains about lifetimes.
            Page::Err(msg, _) => get_error_page(msg.clone()),
            Page::Settings => self.get_settings_page(),
        }
    }
}
