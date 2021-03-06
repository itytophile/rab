use ron::{de::from_reader, Error};
use serde::Deserialize;
use std::ops::Deref;
use std::{collections::HashMap, fs::File};
use std::{fmt::Display, fs};

use rab_core::armor_and_skills::{Armor, Skill};

#[derive(Deserialize, Clone)]
pub struct Locale {
    name: String,
    skills: HashMap<Skill, String>,
    interface: HashMap<InterfaceSymbol, String>,
    armors: HashMap<String, String>,
}

pub fn get_locales(directory_path: &str) -> Result<HashMap<String, Locale>, Error> {
    let mut locales: HashMap<String, Locale> = HashMap::with_capacity(7);
    let paths = fs::read_dir(directory_path)?;

    for path in paths {
        let path = path?.path();
        if let Some(ext) = path.extension() {
            if let Some(ext) = ext.to_str() {
                if ext == "ron" {
                    let locale: Locale = from_reader(File::open(path)?)?;
                    locales.insert(locale.name.clone(), locale);
                }
            }
        }
    }

    Ok(locales)
}

pub trait Localization {
    fn apply_locale(&self, locale: &Option<Locale>) -> String;
}

// Will check the locale. If the localization doesn't exist
// it will use the Debug value.
impl Localization for Skill {
    fn apply_locale(&self, locale: &Option<Locale>) -> String {
        if let Some(locale) = locale {
            if let Some(localized) = locale.skills.get(self) {
                return localized.clone();
            }
        }
        format!("{:?}", self)
    }
}

impl Localization for Armor {
    fn apply_locale(&self, locale: &Option<Locale>) -> String {
        if let Some(locale) = locale {
            if let Some(localized) = locale.armors.get(&self.name) {
                return localized.clone();
            }
        }
        self.name.clone()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum InterfaceSymbol {
    AddWish,
    ManageTalismans,
    SearchBuilds,
    Female,
    Male,
    SkillFilter,
    Remove,
    Settings,
    Helmet,
    Chest,
    Arm,
    Waist,
    Leg,
    Talisman,
    NoResult,
    WeaponSlots,
    AddTalisman,
    Back,
    Edit,
    AddSkill,
    Slots,
    Cancel,
    Save,
    DiscardModifications,
    SaveToFile,
    TalismanName,
    Defense,
    Fire,
    Water,
    Thunder,
    Ice,
    Dragon,
    Free,
    RemoveTalisman,
    TemplateFreeSlot,
    TemplateJewelOnSlot,
    UpdateArmors,
    Updated,
    Updating,
    ProblemCheckConsole,
    ManageBuilds,
    SaveBuild,
    NewBuildName,
}

impl InterfaceSymbol {
    fn default_string(&self) -> String {
        match self {
            InterfaceSymbol::TemplateFreeSlot => "Free lvl {size} slot".to_string(),
            InterfaceSymbol::TemplateJewelOnSlot => "{skill} on lvl {size} slot".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

impl Localization for InterfaceSymbol {
    fn apply_locale(&self, locale: &Option<Locale>) -> String {
        if let Some(locale) = locale {
            if let Some(localized) = locale.interface.get(self) {
                return localized.clone();
            }
        }
        self.default_string()
    }
}

impl Display for InterfaceSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.apply_locale(&*crate::LOCALE.lock().unwrap()))
    }
}

impl From<InterfaceSymbol> for String {
    fn from(sy: InterfaceSymbol) -> Self {
        sy.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LocalizedArmor<'a>(pub &'a Armor);
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct LocalizedSkill(pub Skill);

impl Display for LocalizedArmor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.apply_locale(&*crate::LOCALE.lock().unwrap())
        )
    }
}

impl Display for LocalizedSkill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.apply_locale(&*crate::LOCALE.lock().unwrap())
        )
    }
}

impl Deref for LocalizedArmor<'_> {
    type Target = Armor;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Deref for LocalizedSkill {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
