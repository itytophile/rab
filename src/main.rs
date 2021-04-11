use std::fs::File;
use ron::de::from_reader;

mod armor_ron_description;
use armor_ron_description::Armor;

fn main() {
    let file = File::open("waists.ron").expect("Failed opening file");
    let helmets: Vec<Armor> = from_reader(file).unwrap();

    dbg!(&helmets[0]);
}
