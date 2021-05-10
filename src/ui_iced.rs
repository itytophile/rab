mod builds_page;
mod common_elements;
mod details_page;
mod lang_page;
mod main_page;
mod no_files_page;
mod talisman_page;

use std::{
    collections::HashMap,
    fs::{self, canonicalize},
    path::Path,
};

use crate::{
    armor_and_skills::{
        get_armor_list, get_talismans, save_talismans_to_file, Armor, Gender, Skill,
    },
    build_search::{pre_selection_then_brute_force_search, Build},
    locale::{get_locales, Locale},
    profile::{get_profile, save_profile},
    style_iced,
    update::download_armors_and_locales,
    ARMORS_PATH, LOCALE_DIR_PATH,
};

use iced::{
    button, executor, pick_list, scrollable, slider, text_input, Application, Clipboard, Command,
    Container, Element, Length,
};
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
    Error,
};

use self::{
    builds_page::BuildsPage, details_page::DetailsPage, lang_page::LangPage, main_page::MainPage,
    no_files_page::NoFilesPage, talisman_page::TalismanPage,
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

    sorted_wish_choices: Vec<Skill>,
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

    state_lang_button: button::State,

    locales: HashMap<String, Locale>,
    selected_locale: String,

    state_buttons_locale: Vec<button::State>,

    profile: HashMap<String, String>,

    theme: style_iced::Theme,
    state_theme_button: button::State,

    state_update_button: button::State,
    update_state: UpdateState,

    details_build_index: usize,

    saved_builds: HashMap<String, Build>,

    state_builds_menu_button: button::State,
    states_saved_builds_button: Vec<(
        button::State,
        button::State,
        button::State,
        button::State,
        button::State,
        button::State, //talisman
        button::State, //weapon
        button::State, //remove
    )>,
    details_build_name: String,

    focused_build: Option<Build>,
    total_skills_and_amounts_focused_build: Vec<(Skill, u8)>, // to not sort everytime
}

#[derive(Clone, Copy)]
enum UpdateState {
    Updating,
    Done,
    Initial,
    Problem,
}

impl Default for UpdateState {
    fn default() -> Self {
        UpdateState::Initial
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Talisman,
    Lang,
    NoFiles,
    Details(bool), // true check saved builds
    Builds,
}

impl Default for Page {
    fn default() -> Self {
        Page::Main
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    WishSelected(usize, Skill),
    AddWish,
    RemoveWish(usize),
    SliderChanged(usize, u8),
    Search,
    ArmorDesc(Option<(Armor, [Option<Skill>; 3])>),
    FilterChanged(String),
    GenderChanged(Gender),
    WeaponSlotChanged(usize, u8),
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
    ToggleTheme,
    UpdateArmors,
    UpdateDone(bool), // true = no problem
    DownloadArmors,
    DownloadDone(bool),
    BuildDetails(usize), // index of build in vec builds
    SaveBuild(usize),
    SavedBuildDetails(String), // index of build in vec builds
    EditSavedBuild(String),
    RemoveSavedBuild(String),
}

const WAISTS_PATH: &str = "armors/waists.ron";
const HELMETS_PATH: &str = "armors/helmets.ron";
const ARMS_PATH: &str = "armors/arms.ron";
const LEGS_PATH: &str = "armors/legs.ron";
const CHESTS_PATH: &str = "armors/chests.ron";

const TALISMANS_PATH: &str = "talismans.ron";
const PROFILE_PATH: &str = "profile.ron";
const BUILDS_PATH: &str = "builds.ron";

impl MainApp {
    fn clear_talisman_editor(&mut self) {
        for (_, slider_value) in self.states_values_slider_talisman_slot.iter_mut() {
            *slider_value = 0
        }
        self.edit_wish_fields.clear();
    }

    fn save_profile(&self) {
        match save_profile(&self.profile, PROFILE_PATH) {
            Ok(file) => println!("Profile saved to {}", file),
            Err(err) => println!("Can't save profile:\n{}", err),
        };
    }

    fn reload_locales(&mut self) {
        self.locales = match get_locales(LOCALE_DIR_PATH) {
            Ok(locales) => locales,
            Err(err) => {
                println!(
                    "Error with localization files at {}\n{}\nWARNING: no locale loaded.",
                    LOCALE_DIR_PATH, err
                );
                HashMap::with_capacity(0)
            }
        };

        self.state_buttons_locale = vec![Default::default(); self.locales.len()];

        *super::LOCALE.lock().unwrap() = self.locales.get(&self.selected_locale).cloned();
    }

    fn reload_armors(&mut self) {
        let (helmets, chests, arms, waists, legs) = match get_all_armors_from_file() {
            Ok(lists) => lists,
            Err(err) => {
                println!("ERROR: Can't reload armors:\n{}", err);
                (vec![], vec![], vec![], vec![], vec![])
            }
        };
        self.helmets = helmets;
        self.chests = chests;
        self.arms = arms;
        self.waists = waists;
        self.legs = legs;
    }

    fn save_builds(&self) {
        match save_builds(&self.saved_builds, BUILDS_PATH) {
            Ok(path) => println!("Builds saved to {}", path),
            Err(err) => println!("Unable to save builds:\n{}", err),
        }
    }

    fn focus_new_build(&mut self, build: Build) {
        self.total_skills_and_amounts_focused_build =
            build.get_all_skills_and_amounts().drain().collect();
        self.total_skills_and_amounts_focused_build
            .sort_unstable_by_key(|(_, amount)| *amount);
        self.focused_build = Some(build);
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

fn create_locale_and_armors_dir() {
    let armors_path = Path::new(ARMORS_PATH);
    if !armors_path.is_dir() {
        match fs::create_dir(armors_path) {
            Err(err) => println!("Can't create armors directory:\n{}", err),
            _ => {}
        }
    }
    let locale_path = Path::new(LOCALE_DIR_PATH);
    if !locale_path.is_dir() {
        match fs::create_dir(locale_path) {
            Err(err) => println!("Can't create locale directory:\n{}", err),
            _ => {}
        }
    }
}

use lexical_sort::natural_lexical_cmp;

fn save_builds(builds: &HashMap<String, Build>, path: &str) -> Result<String, Error> {
    let text = to_string_pretty(builds, PrettyConfig::new().with_indentor("  ".to_string()))?;

    fs::write(path, text)?;

    let path = canonicalize(path)?;

    Ok(path.to_string_lossy().into_owned())
}

fn get_saved_builds(path: &str) -> Result<HashMap<String, Build>, Error> {
    Ok(from_reader(fs::File::open(path)?)?)
}

impl Application for MainApp {
    type Message = Msg;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Msg>) {
        // let text = iced_futures::futures::executor::block_on(get_arms());
        // println!("{}", text);
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
            Err(_) => {
                page = Page::NoFiles;
                (vec![], vec![], vec![], vec![], vec![])
            }
        };

        let locales = match get_locales(LOCALE_DIR_PATH) {
            Ok(locales) => locales,
            Err(err) => {
                println!(
                    "Error with localization files at {}\n{}\nWARNING: no locale loaded.",
                    LOCALE_DIR_PATH, err
                );
                HashMap::with_capacity(0)
            }
        };

        let profile = match get_profile(PROFILE_PATH) {
            Ok(map) => {
                println!("Profile file succesfully loaded.");
                map
            }
            Err(err) => {
                println!(
                    "Can't read the profile file:\n{}\nEmpty profile loaded.",
                    err
                );
                Default::default()
            }
        };

        let selected_locale = profile
            .get("lang")
            .cloned()
            .unwrap_or("English".to_string());

        let theme = match profile.get("theme").unwrap_or(&"dark".to_string()).as_str() {
            "light" => style_iced::Theme::Light,
            _ => style_iced::Theme::Dark,
        };

        let state_buttons_locale = vec![Default::default(); locales.len()];

        *super::LOCALE.lock().unwrap() = locales.get(&selected_locale).cloned();

        let mut sorted_wish_choices = Skill::ALL.to_vec();
        sorted_wish_choices
            .sort_unstable_by(|a, b| natural_lexical_cmp(&a.to_string(), &b.to_string()));

        let filtered_wish_choices = sorted_wish_choices.clone();

        let saved_builds = match get_saved_builds(BUILDS_PATH) {
            Ok(map) => {
                println!("Builds file succesfully loaded.");
                map
            }
            Err(err) => {
                println!("Can't read the builds file:\n{}\nNo builds loaded.", err);
                Default::default()
            }
        };

        let states_saved_builds_button = vec![Default::default(); saved_builds.len()];

        (
            Self {
                wish_fields: vec![WishField::default()],

                waists,
                helmets,
                arms,
                legs,
                chests,
                talismans,
                states_talisman_button,

                filtered_wish_choices,
                sorted_wish_choices,

                selected_gender: Gender::Female,

                page,

                locales,
                selected_locale,

                state_buttons_locale,

                profile,

                theme,

                saved_builds,
                states_saved_builds_button,

                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("RAB - Rusty Armor Builds")
    }

    fn update(&mut self, message: Msg, _clipboard: &mut Clipboard) -> Command<Msg> {
        match message {
            Msg::WishSelected(key, wish) => {
                self.wish_fields[key].selected = wish;
                self.wish_fields[key].value_slider = 1
            }
            Msg::AddWish => self.wish_fields.push(WishField::default()),
            Msg::RemoveWish(index) => {
                self.wish_fields.remove(index);
            }
            Msg::SliderChanged(index, value) => self.wish_fields[index].value_slider = value,
            Msg::Search => {
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
                    [
                        self.states_values_slider_weapon_slot[0].1,
                        self.states_values_slider_weapon_slot[1].1,
                        self.states_values_slider_weapon_slot[2].1,
                    ],
                );
                self.states_build_button = vec![Default::default(); self.builds.len()];
            }
            Msg::ArmorDesc(option) => self.armor_desc = option,
            Msg::FilterChanged(text) => {
                self.value_filter_text_input = text;
                self.filtered_wish_choices = self
                    .sorted_wish_choices
                    .iter()
                    .copied()
                    .filter(|skill| {
                        skill
                            .to_string()
                            .to_lowercase()
                            .contains(&self.value_filter_text_input.to_lowercase())
                    })
                    .collect();
            }
            Msg::GenderChanged(gender) => self.selected_gender = gender,
            Msg::WeaponSlotChanged(index, value) => {
                self.states_values_slider_weapon_slot[index].1 = value
            }
            Msg::SelectTalisman(index) => self.selected_talisman = index,
            Msg::EditTalisman => {
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
            Msg::SaveEdition => {
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
            Msg::CancelEdition => {
                self.is_editing = false;
                self.clear_talisman_editor()
            }
            Msg::TalismanSlotChanged(index, value) => {
                self.states_values_slider_talisman_slot[index].1 = value
            }
            Msg::EditTalismanName(name) => self.value_edit_text_input = name,
            Msg::EditSkillSelected(key, wish) => {
                self.edit_wish_fields[key].selected = wish;
                self.edit_wish_fields[key].value_slider = 1
            }
            Msg::EditAddSkill => self.edit_wish_fields.push(WishField::default()),
            Msg::EditRemoveSkill(index) => {
                self.edit_wish_fields.remove(index);
            }
            Msg::EditSkillSliderChanged(index, value) => {
                self.edit_wish_fields[index].value_slider = value
            }
            Msg::RemoveTalisman => {
                let index = self.selected_talisman.unwrap();
                self.talismans.remove(index);
                self.states_talisman_button.remove(index);
                self.clear_talisman_editor();
                self.is_editing = false;
                self.selected_talisman = None;
            }
            Msg::AddTalisman => {
                self.talismans.push(Armor {
                    name: "New talisman".to_string(),
                    skills: vec![(Skill::Botanist, 1)],
                    ..Default::default()
                });
                self.states_talisman_button.push(Default::default())
            }
            Msg::SaveTalismans => match save_talismans_to_file(&self.talismans, TALISMANS_PATH) {
                Ok(path) => println!("Talismans saved to {}", path),
                Err(err) => println!("Unable to save the talismans: {}", err),
            },
            Msg::DiscardTalismans => {
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
            Msg::ChangePage(page) => self.page = page,
            Msg::LocaleChanged(new_locale) => {
                self.profile.insert("lang".to_string(), new_locale.clone());
                *super::LOCALE.lock().unwrap() = self.locales.get(&new_locale).cloned();
                // after the unwrap(), if there is a mystical problem with the mutex
                // it is better to not save the locale
                self.save_profile();
                self.sorted_wish_choices
                    .sort_unstable_by(|a, b| natural_lexical_cmp(&a.to_string(), &b.to_string()));

                self.filtered_wish_choices = self
                    .sorted_wish_choices
                    .iter()
                    .copied()
                    .filter(|skill| {
                        skill
                            .to_string()
                            .to_ascii_lowercase()
                            .contains(&self.value_filter_text_input.to_ascii_lowercase())
                    })
                    .collect();

                self.selected_locale = new_locale;
            }
            Msg::ToggleTheme => {
                self.theme = match self.theme {
                    style_iced::Theme::Dark => {
                        self.profile
                            .insert("theme".to_string(), "light".to_string());
                        style_iced::Theme::Light
                    }
                    _ => {
                        self.profile.insert("theme".to_string(), "dark".to_string());
                        style_iced::Theme::Dark
                    }
                };
                self.save_profile()
            }
            Msg::UpdateArmors => {
                self.update_state = UpdateState::Updating;
                create_locale_and_armors_dir();
                return Command::perform(download_armors_and_locales(), |no_problem| {
                    Msg::UpdateDone(no_problem)
                });
            }
            Msg::UpdateDone(no_problem) => {
                self.update_state = if no_problem {
                    self.reload_locales();
                    self.reload_armors();

                    UpdateState::Done
                } else {
                    UpdateState::Problem
                }
            }
            Msg::DownloadArmors => {
                self.update_state = UpdateState::Updating;
                create_locale_and_armors_dir();
                return Command::perform(download_armors_and_locales(), |no_problem| {
                    Msg::DownloadDone(no_problem)
                });
            }
            Msg::DownloadDone(no_problem) => {
                self.update_state = if no_problem {
                    self.reload_locales();
                    self.reload_armors();

                    self.page = Page::Lang;
                    UpdateState::Done
                } else {
                    UpdateState::Problem
                }
            }
            Msg::BuildDetails(index) => {
                self.value_edit_text_input = "".to_string();

                self.focus_new_build(self.builds[index].clone());

                self.details_build_index = index;
                self.page = Page::Details(false)
            }
            Msg::SaveBuild(index) => {
                if self
                    .saved_builds
                    .insert(
                        self.value_edit_text_input.clone(),
                        self.builds[index].clone(),
                    )
                    .is_none()
                {
                    self.states_saved_builds_button.push(Default::default())
                };

                self.page = Page::Builds;
                self.save_builds();
            }
            Msg::SavedBuildDetails(name) => {
                self.value_edit_text_input = name.clone();

                self.focus_new_build(self.saved_builds.get(&name).unwrap().clone());

                self.details_build_name = name;
                self.page = Page::Details(true)
            }
            Msg::EditSavedBuild(name) => {
                let build = self.saved_builds.remove(&name).unwrap();
                self.saved_builds
                    .insert(self.value_edit_text_input.clone(), build);

                self.details_build_name = self.value_edit_text_input.clone();

                self.page = Page::Builds;
                self.save_builds();
            }
            Msg::RemoveSavedBuild(name) => {
                self.saved_builds.remove(&name);
                self.save_builds();
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Msg> {
        let theme = self.theme;

        let container = Container::new(match self.page {
            Page::Main => self.get_main_page(),
            Page::Talisman => self.get_talisman_page(),
            Page::NoFiles => self.get_no_files_page(),
            Page::Lang => self.get_lang_page(),
            Page::Details(on_save_builds) => self.get_details_page(on_save_builds),
            Page::Builds => self.get_builds_page(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        match theme {
            style_iced::Theme::Dark => container.style(style_iced::Container::DarkTheme),
            style_iced::Theme::Light => container,
        }
        .into()
    }
}
