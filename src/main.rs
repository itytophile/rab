mod armor_ron;
mod build_search;

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

fn debug_build_part(part: &Option<(Armor, Jewels)>) -> String {
    match part {
        None => "None".to_string(),
        Some((armor, jewels)) => format!("{}:{:?}", armor.name, jewels),
    }
}