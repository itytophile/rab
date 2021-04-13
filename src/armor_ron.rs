use lazy_static::lazy_static;
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize, Clone)]
pub enum Gender {
    Female,
    Male,
    Neutral,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Armor {
    pub name: String,
    pub skills: Vec<(Skill, u8)>,
    pub slots: Vec<u8>,
    pub rare: u8,
    pub defense: u8,
    pub fire: i8,
    pub water: i8,
    pub thunder: i8,
    pub ice: i8,
    pub dragon: i8,
    pub gender: Gender,
}

pub fn get_armor_list(path: &str) -> Vec<Armor> {
    from_reader(File::open(path).expect(&format!("Failed opening {}", path))).unwrap()
}

pub struct SkillDesc {
    pub limit: u8,
    pub jewel_size: Option<u8>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Skill {
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
    pub static ref SKILL_LIMIT_JEWEL_SIZE: HashMap<Skill, SkillDesc> = {
        let mut m = HashMap::new();
        use Skill::*;
        m.insert(
            Botanist,
            SkillDesc {
                limit: 4,
                jewel_size: Some(1),
            },
        );
        m.insert(
            DefenseBoost,
            SkillDesc {
                limit: 7,
                jewel_size: Some(1),
            },
        );
        m.insert(
            ItemProlonger,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            CriticalEye,
            SkillDesc {
                limit: 7,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Fortify,
            SkillDesc {
                limit: 1,
                jewel_size: Some(2),
            },
        );
        m.insert(
            PoisonAttack,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            RecoilDown,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            QuickSheath,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            FireAttack,
            SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },
        );
        m.insert(
            IceAttack,
            SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },
        );
        m.insert(
            WaterAttack,
            SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },
        );
        m.insert(
            ProtectivePolish,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            StaminaThief,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            Partbreaker,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            Mushroomancer,
            SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },
        );
        m.insert(
            MaximumMight,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            MarathonRunner,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            PeakPerformance,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            AttackBoost,
            SkillDesc {
                limit: 7,
                jewel_size: Some(2),
            },
        );
        m.insert(
            OffensiveGuard,
            SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },
        );
        m.insert(
            Focus,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            RecoveryUp,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            NormalRapidUp,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            SpeedEating,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Windproof,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            Bludgeoner,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            AffinitySliding,
            SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
        );
        m.insert(
            WideRange,
            SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },
        );
        m.insert(
            StunResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            LoadShells,
            SkillDesc {
                limit: 2,
                jewel_size: Some(2),
            },
        );
        m.insert(
            ParalysisAttack,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            PierceUp,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            AimBooster,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            SleepAttack,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            BlightResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            CriticalDraw,
            SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },
        );
        m.insert(
            JumpMaster,
            SkillDesc {
                limit: 1,
                jewel_size: None,
            },
        );
        m.insert(
            Constitution,
            SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },
        );
        m.insert(
            FreeMeal,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            GoodLuck,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            RazorSharp,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            SpareShot,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            WirebugWhisperer,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Resentment,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            Handicraft,
            SkillDesc {
                limit: 5,
                jewel_size: None,
            },
        );
        m.insert(
            FlinchFree,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            RapidMorph,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            LatentPower,
            SkillDesc {
                limit: 5,
                jewel_size: None,
            },
        );
        m.insert(
            WeaknessExploit,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            Resuscitate,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            EvadeWindow,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Slugger,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            SpecialAmmoBoost,
            SkillDesc {
                limit: 2,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Agitator,
            SkillDesc {
                limit: 5,
                jewel_size: None,
            },
        );
        m.insert(
            DevineBlessing,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Geologist,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            HungerResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            CriticalElement,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            EvadeExtender,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            DragonAttack,
            SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },
        );
        m.insert(
            Heroics,
            SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },
        );
        m.insert(
            SleepResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            ParalysisResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            PoisonResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            WindAlignment,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            SpreadUp,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            ReloadSpeed,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            ThunderAlignment,
            SkillDesc {
                limit: 5,
                jewel_size: None,
            },
        );
        m.insert(
            Guard,
            SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },
        );
        m.insert(
            StaminaSurge,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Earplugs,
            SkillDesc {
                limit: 5,
                jewel_size: Some(3),
            },
        );
        m.insert(
            BowChargePlus,
            SkillDesc {
                limit: 1,
                jewel_size: None,
            },
        );
        m.insert(
            BlastResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            AmmoUp,
            SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },
        );
        m.insert(
            LeapofFaith,
            SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
        );
        m.insert(
            DragonResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            WaterResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            DivineBlessing,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            RecoverySpeed,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            SpeedSharpening,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            MuckResistance,
            SkillDesc {
                limit: 2,
                jewel_size: Some(1),
            },
        );
        m.insert(
            PowerProlonger,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            TremorResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            HellfireCloak,
            SkillDesc {
                limit: 4,
                jewel_size: None,
            },
        );
        m.insert(
            BubblyDance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            PunishingDraw,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            WallRunner,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            GuardUp,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            CriticalBoost,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            MindsEye,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            BlastAttack,
            SkillDesc {
                limit: 3,
                jewel_size: None,
            },
        );
        m.insert(
            MasterMounter,
            SkillDesc {
                limit: 1,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Counterstrike,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            ThunderAttack,
            SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },
        );
        m.insert(
            Artillery,
            SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
        );
        m.insert(
            Bombardier,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            CaptureMaster,
            SkillDesc {
                limit: 1,
                jewel_size: None,
            },
        );
        m.insert(
            Diversion,
            SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
        );
        m.insert(
            FireResistance,
            SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },
        );
        m.insert(
            HornMaestro,
            SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
        );
        m
    };
}
