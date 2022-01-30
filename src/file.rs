use std::fs::{self, File};

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
    Error,
};

use rab_core::armor_and_skills::{armor_to_talisman, talisman_to_armor, Armor, Talisman};

pub fn save_talismans_to_file(talismans: &[Armor], path: &str) -> Result<String, Error> {
    let talismans: Vec<Talisman> = talismans.iter().map(armor_to_talisman).collect();

    let text = to_string_pretty(&talismans, PrettyConfig::new().indentor("  ".to_string()))?;

    fs::write(path, text)?;

    let path = fs::canonicalize(path)?;

    Ok(path.to_string_lossy().into_owned())
}

pub fn get_armor_list(path: &str) -> Result<Vec<Armor>, Error> {
    let armors: Vec<Armor> = from_reader(File::open(path)?)?;
    Ok(armors)
}

pub fn get_talismans(path: &str) -> Result<Vec<Armor>, Error> {
    let file = File::open(path)?;
    let talismans: Vec<Talisman> = from_reader(file)?;
    let talismans: Vec<Armor> = talismans.iter().map(talisman_to_armor).collect();
    Ok(talismans)
}
