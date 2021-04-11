mod armor_ron;
use armor_ron::{Armor, get_armor_list};

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
}
