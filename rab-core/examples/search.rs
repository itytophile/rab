use rab_core::{
    armor_and_skills::{Armor, Skill},
    build_search::{pre_selection_then_brute_force_search, AllArmorSlices},
};
use ron::de::{from_reader, Error};
use std::{fs::File, path::Path};

fn main() {
    let helmets = &get_armors("../armors/helmets.ron").unwrap();
    let chests = &get_armors("../armors/chests.ron").unwrap();
    let arms = &get_armors("../armors/arms.ron").unwrap();
    let waists = &get_armors("../armors/waists.ron").unwrap();
    let legs = &get_armors("../armors/legs.ron").unwrap();

    dbg!(pre_selection_then_brute_force_search(
        &[
            (Skill::QuickSheath, 3),
            (Skill::CriticalDraw, 3),
            (Skill::CriticalBoost, 3),
            (Skill::WeaknessExploit, 2),
        ],
        AllArmorSlices {
            helmets,
            chests,
            arms,
            waists,
            legs,
            talismans: &[],
        },
        rab_core::armor_and_skills::Gender::Male,
        [0; 3],
    )
    .len());
}

fn get_armors(path: impl AsRef<Path>) -> Result<Vec<Armor>, Error> {
    from_reader(File::open(path)?)
}
