use lazy_static::lazy_static;
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
enum Skill {
    Botanist,
    DefenseBoost,
    ItemProlonger,
    CriticalEye,
    Fortify,
    PoisonAttack,
    RecoilDown,
    QuickSheath,
    FireAttack,
    IceAttack,
    WaterAttack,
    ProtectivePolish,
    StaminaThief,
    Partbreaker,
    Mushroomancer,
    MaximumMight,
    MarathonRunner,
    PeakPerformance,
    AttackBoost,
    OffensiveGuard,
    Focus,
    RecoveryUp,
    NormalRapidUp,
    SpeedEating,
    Windproof,
    Bludgeoner,
    AffinitySliding,
    WideRange,
    StunResistance,
    LoadShells,
    ParalysisAttack,
    PierceUp,
    AimBooster,
    SleepAttack,
    BlightResistance,
    CriticalDraw,
    JumpMaster,
    Constitution,
    FreeMeal,
    GoodLuck,
    RazorSharp,
    SpareShot,
    WirebugWhisperer,
    Resentment,
    Handicraft,
    FlinchFree,
    RapidMorph,
    LatentPower,
    WeaknessExploit,
    Resuscitate,
    EvadeWindow,
    Slugger,
    SpecialAmmoBoost,
    Agitator,
    DevineBlessing,
    Geologist,
    HungerResistance,
    CriticalElement,
    EvadeExtender,
    DragonAttack,
    Heroics,
    SleepResistance,
    ParalysisResistance,
    PoisonResistance,
    WindAlignment,
    SpreadUp,
    ReloadSpeed,
    ThunderAlignment,
    Guard,
    StaminaSurge,
    Earplugs,
    BowChargePlus,
    BlastResistance,
    AmmoUp,
    LeapofFaith,
    DragonResistance,
    WaterResistance,
    DivineBlessing,
    RecoverySpeed,
    SpeedSharpening,
    MuckResistance,
    PowerProlonger,
    TremorResistance,
    HellfireCloak,
    BubblyDance,
    PunishingDraw,
    WallRunner,
    GuardUp,
    CriticalBoost,
    MindsEye,
    BlastAttack,
    MasterMounter,
    Counterstrike,
    ThunderAttack,
    Artillery,
    Bombardier,
    CaptureMaster,
    Diversion,
    FireResistance,
    HornMaestro,
}

lazy_static! {
    static ref SKILL_LIMIT: HashMap<Skill, u8> = {
        let mut m = HashMap::new();
        m.insert(Skill::Botanist, 4);
        m.insert(Skill::DefenseBoost, 7);
        m.insert(Skill::ItemProlonger, 3);
        m.insert(Skill::CriticalEye, 7);
        m.insert(Skill::Fortify, 1);
        m.insert(Skill::PoisonAttack, 3);
        m.insert(Skill::RecoilDown, 3);
        m.insert(Skill::QuickSheath, 3);
        m.insert(Skill::FireAttack, 5);
        m.insert(Skill::IceAttack, 5);
        m.insert(Skill::WaterAttack, 5);
        m.insert(Skill::ProtectivePolish, 3);
        m.insert(Skill::StaminaThief, 3);
        m.insert(Skill::Partbreaker, 3);
        m.insert(Skill::Mushroomancer, 3);
        m.insert(Skill::MaximumMight, 3);
        m.insert(Skill::MarathonRunner, 3);
        m.insert(Skill::PeakPerformance, 3);
        m.insert(Skill::AttackBoost, 7);
        m.insert(Skill::OffensiveGuard, 3);
        m.insert(Skill::Focus, 3);
        m.insert(Skill::RecoveryUp, 3);
        m.insert(Skill::NormalRapidUp, 3);
        m.insert(Skill::SpeedEating, 3);
        m.insert(Skill::Windproof, 3);
        m.insert(Skill::Bludgeoner, 3);
        m.insert(Skill::AffinitySliding, 1);
        m.insert(Skill::WideRange, 5);
        m.insert(Skill::StunResistance, 3);
        m.insert(Skill::LoadShells, 2);
        m.insert(Skill::ParalysisAttack, 3);
        m.insert(Skill::PierceUp, 3);
        m.insert(Skill::AimBooster, 3);
        m.insert(Skill::SleepAttack, 3);
        m.insert(Skill::BlightResistance, 3);
        m.insert(Skill::CriticalDraw, 3);
        m.insert(Skill::JumpMaster, 1);
        m.insert(Skill::Constitution, 5);
        m.insert(Skill::FreeMeal, 3);
        m.insert(Skill::GoodLuck, 3);
        m.insert(Skill::RazorSharp, 3);
        m.insert(Skill::SpareShot, 3);
        m.insert(Skill::WirebugWhisperer, 3);
        m.insert(Skill::Resentment, 5);
        m.insert(Skill::Handicraft, 5);
        m.insert(Skill::FlinchFree, 3);
        m.insert(Skill::RapidMorph, 3);
        m.insert(Skill::LatentPower, 5);
        m.insert(Skill::WeaknessExploit, 3);
        m.insert(Skill::Resuscitate, 3);
        m.insert(Skill::EvadeWindow, 5);
        m.insert(Skill::Slugger, 3);
        m.insert(Skill::SpecialAmmoBoost, 2);
        m.insert(Skill::Agitator, 5);
        m.insert(Skill::DevineBlessing, 3);
        m.insert(Skill::Geologist, 3);
        m.insert(Skill::HungerResistance, 3);
        m.insert(Skill::CriticalElement, 3);
        m.insert(Skill::EvadeExtender, 3);
        m.insert(Skill::DragonAttack, 5);
        m.insert(Skill::Heroics, 5);
        m.insert(Skill::SleepResistance, 3);
        m.insert(Skill::ParalysisResistance, 3);
        m.insert(Skill::PoisonResistance, 3);
        m.insert(Skill::WindAlignment, 5);
        m.insert(Skill::SpreadUp, 3);
        m.insert(Skill::ReloadSpeed, 3);
        m.insert(Skill::ThunderAlignment, 5);
        m.insert(Skill::Guard, 5);
        m.insert(Skill::StaminaSurge, 3);
        m.insert(Skill::Earplugs, 5);
        m.insert(Skill::BowChargePlus, 1);
        m.insert(Skill::BlastResistance, 3);
        m.insert(Skill::AmmoUp, 3);
        m.insert(Skill::LeapofFaith, 1);
        m.insert(Skill::DragonResistance, 3);
        m.insert(Skill::WaterResistance, 3);
        m.insert(Skill::DivineBlessing, 3);
        m.insert(Skill::RecoverySpeed, 3);
        m.insert(Skill::SpeedSharpening, 3);
        m.insert(Skill::MuckResistance, 2);
        m.insert(Skill::PowerProlonger, 3);
        m.insert(Skill::TremorResistance, 3);
        m.insert(Skill::HellfireCloak, 4);
        m.insert(Skill::BubblyDance, 3);
        m.insert(Skill::PunishingDraw, 3);
        m.insert(Skill::WallRunner, 3);
        m.insert(Skill::GuardUp, 3);
        m.insert(Skill::CriticalBoost, 3);
        m.insert(Skill::MindsEye, 3);
        m.insert(Skill::BlastAttack, 3);
        m.insert(Skill::MasterMounter, 1);
        m.insert(Skill::Counterstrike, 3);
        m.insert(Skill::ThunderAttack, 5);
        m.insert(Skill::Artillery, 3);
        m.insert(Skill::Bombardier, 3);
        m.insert(Skill::CaptureMaster, 1);
        m.insert(Skill::Diversion, 1);
        m.insert(Skill::FireResistance, 3);
        m.insert(Skill::HornMaestro, 1);
        m
    };
}


#[derive(Debug, Deserialize)]
enum Gender {
    Female,
    Male,
    Neutral,
}

#[derive(Debug, Deserialize)]
pub struct Armor {
    name: String,
    skills: Vec<(Skill, u8)>,
    slots: Vec<u8>,
    rare: u8,
    defense: u8,
    fire: i8,
    water: i8,
    thunder: i8,
    ice: i8,
    dragon: i8,
    gender: Gender,
}

pub fn get_armor_list(path: &str) -> Vec<Armor> {
    from_reader(File::open(path).expect(&format!("Failed opening {}", path))).unwrap()
}
