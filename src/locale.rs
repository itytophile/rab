use ron::{de::from_reader, Error};
use serde::Deserialize;
use std::fs;
use std::{collections::HashMap, fs::File};

use crate::armor_and_skills::Skill;

#[derive(Deserialize, Clone)]
pub struct Locale {
    name: String,
    skills: HashMap<Skill, String>,
}

pub fn get_locales(directory_path: &str) -> Result<HashMap<String, Locale>, Error> {
    let mut locales: HashMap<String, Locale> = HashMap::with_capacity(2);
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

impl Localization for Skill {
    fn apply_locale(&self, locale: &Option<Locale>) -> String {
        if let Some(locale) = locale {
            if let Some(localized) = locale.skills.get(self) {
                localized.clone()
            } else {
                format!("{:?}", self)
            }
        } else {
            format!("{:?}", self)
        }
    }
}
