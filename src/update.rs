use std::{fs, array, path::Path};

use crate::{ARMORS_PATH, LOCALE_DIR_PATH};

const ARMORS_BASE_URL: &str =
    "https://raw.githubusercontent.com/itytophile/monster-hunter-rise-armors/main/";
const HELMETS_FILE: &str = "helmets.ron";
const CHESTS_FILE: &str = "chests.ron";
const ARMS_FILE: &str = "arms.ron";
const WAISTS_FILE: &str = "waists.ron";
const LEGS_FILE: &str = "legs.ron";

async fn download_file(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let txt = resp.text().await?;
    Ok(txt)
}

async fn download_and_write_file(base_url: &str, file: &str, dir: impl AsRef<Path>) -> bool {
    match download_file(&format!("{}{}", base_url, file)).await {
        Ok(text) => match fs::write(dir.as_ref().join(file), text) {
            Err(err) => {
                println!("Can't write file '{}':\n{}", file, err);
                false
            }
            _ => {
                println!("{} updated!", file);
                true
            }
        },
        Err(err) => {
            println!("Error with download:\n{}", err);
            false
        }
    }
}

async fn download_armors() -> bool {
    let futures =
        array::IntoIter::new([HELMETS_FILE, CHESTS_FILE, ARMS_FILE, WAISTS_FILE, LEGS_FILE])
            .map(|file| download_and_write_file(ARMORS_BASE_URL, file, ARMORS_PATH));

    let bools = iced_futures::futures::future::join_all(futures).await;
    // all true = no problem
    bools.iter().all(|b| *b)
}

const LOCALE_BASE_URL: &str = "https://raw.githubusercontent.com/itytophile/rab-locale/main/";

const ENGLISH_LOCALE: &str = "english.ron";
const FRENCH_LOCALE: &str = "french.ron";
const GERMAN_LOCALE: &str = "german.ron";
const ITALIAN_LOCALE: &str = "italian.ron";
const POLISH_LOCALE: &str = "polish.ron";
const RUSSIAN_LOCALE: &str = "russian.ron";
const SPANISH_LOCALE: &str = "spanish.ron";

async fn download_locales() -> bool {
    let futures = array::IntoIter::new([
        ENGLISH_LOCALE,
        FRENCH_LOCALE,
        GERMAN_LOCALE,
        ITALIAN_LOCALE,
        POLISH_LOCALE,
        RUSSIAN_LOCALE,
        SPANISH_LOCALE,
    ])
    .map(|file| download_and_write_file(LOCALE_BASE_URL, file, LOCALE_DIR_PATH));
    let bools = iced_futures::futures::future::join_all(futures).await;
    // all true = no problem
    bools.iter().all(|b| *b)
}

pub async fn download_armors_and_locales() -> bool {
    let bools = iced_futures::futures::future::join(download_armors(), download_locales()).await;
    bools.0 && bools.1
}
