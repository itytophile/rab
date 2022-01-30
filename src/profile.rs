use std::{
    collections::HashMap,
    fs::{canonicalize, write, File},
};

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
    Error,
};

// this profile will be useful for saving user
// preferences after closing RAB, like the language
pub fn get_profile(path: &str) -> Result<HashMap<String, String>, Error> {
    from_reader(File::open(path)?)
}

pub fn save_profile(profile: &HashMap<String, String>, path: &str) -> Result<String, Error> {
    let text = to_string_pretty(profile, PrettyConfig::new().indentor("  ".to_string()))?;

    write(path, text)?;

    let path = canonicalize(path)?;

    Ok(path.to_string_lossy().into_owned())
}
