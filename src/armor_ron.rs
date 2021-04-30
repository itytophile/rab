use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
    Error,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
    Neutral,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Neutral
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize)]
struct Talisman {
    name: String,
    skills: Vec<(Skill, u8)>,
    slots: Vec<u8>,
}

pub fn save_talismans_to_file(talismans: &[Armor], path: &str) -> Result<String, Error> {
    let talismans: Vec<Talisman> = talismans.iter().map(armor_to_talisman).collect();

    let text = to_string_pretty(
        &talismans,
        PrettyConfig::new().with_indentor("  ".to_string()),
    )?;

    fs::write(path, text)?;

    let path = fs::canonicalize(path)?;

    Ok(path.to_string_lossy().into_owned())
}

impl PartialEq for Armor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn get_armor_list(path: &str) -> Result<Vec<Armor>, Error> {
    let armors: Vec<Armor> = from_reader(File::open(path)?)?;
    Ok(armors)
}

fn talisman_to_armor(talisman: &Talisman) -> Armor {
    Armor {
        name: talisman.name.clone(),
        skills: talisman.skills.clone(),
        slots: talisman.slots.clone(),
        ..Default::default()
    }
}

fn armor_to_talisman(armor: &Armor) -> Talisman {
    Talisman {
        name: armor.name.clone(),
        skills: armor.skills.clone(),
        slots: armor.slots.clone(),
    }
}

pub fn get_talismans(path: &str) -> Result<Vec<Armor>, Error> {
    let file = File::open(path)?;
    let talismans: Vec<Talisman> = from_reader(file)?;
    let talismans: Vec<Armor> = talismans.iter().map(talisman_to_armor).collect();
    Ok(talismans)
}

struct SkillDesc {
    pub limit: u8,
    pub jewel_size: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)]
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
    Ballistics,
    KushalaBlessing,
    ChameleosBlessing,
    TeostraBlessing,
    MastersTouch,
    RapidFireUp,
    CarvingPro,
}

use Skill::*;

impl Default for Skill {
    fn default() -> Self {
        Botanist
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Skill {
    pub fn get_jewel_size(&self) -> Option<u8> {
        self.get_skill_desc().jewel_size
    }
    pub fn get_limit(&self) -> u8 {
        self.get_skill_desc().limit
    }
    pub const ALL: [Skill; 107] = [
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
        Ballistics,
        KushalaBlessing,
        ChameleosBlessing,
        TeostraBlessing,
        MastersTouch,
        RapidFireUp,
        CarvingPro,
    ];
    fn get_skill_desc(&self) -> SkillDesc {
        match self {
            Botanist => SkillDesc {
                limit: 4,
                jewel_size: Some(1),
            },

            DefenseBoost => SkillDesc {
                limit: 7,
                jewel_size: Some(1),
            },

            ItemProlonger => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            CriticalEye => SkillDesc {
                limit: 7,
                jewel_size: Some(2),
            },

            Fortify => SkillDesc {
                limit: 1,
                jewel_size: Some(2),
            },

            PoisonAttack => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            RecoilDown => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            QuickSheath => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            FireAttack => SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },

            IceAttack => SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },

            WaterAttack => SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },

            ProtectivePolish => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            StaminaThief => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            Partbreaker => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Mushroomancer => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            MaximumMight => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            MarathonRunner => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            PeakPerformance => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            AttackBoost => SkillDesc {
                limit: 7,
                jewel_size: Some(2),
            },

            OffensiveGuard => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            Focus => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            RecoveryUp => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            NormalRapidUp => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            SpeedEating => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Windproof => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            Bludgeoner => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            AffinitySliding => SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },

            WideRange => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            StunResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            LoadShells => SkillDesc {
                limit: 2,
                jewel_size: Some(2),
            },

            ParalysisAttack => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            PierceUp => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            AimBooster => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            SleepAttack => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            BlightResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            CriticalDraw => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            JumpMaster => SkillDesc {
                limit: 1,
                jewel_size: Some(3),
            },

            Constitution => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            FreeMeal => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            GoodLuck => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            RazorSharp => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            SpareShot => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            WirebugWhisperer => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Resentment => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Handicraft => SkillDesc {
                limit: 5,
                jewel_size: Some(3),
            },

            FlinchFree => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            RapidMorph => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            LatentPower => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            WeaknessExploit => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Resuscitate => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            EvadeWindow => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Slugger => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            SpecialAmmoBoost => SkillDesc {
                limit: 2,
                jewel_size: Some(2),
            },

            Agitator => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            DevineBlessing => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Geologist => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            HungerResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            CriticalElement => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            EvadeExtender => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            DragonAttack => SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },

            Heroics => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            SleepResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            ParalysisResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            PoisonResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            WindAlignment => SkillDesc {
                limit: 3,
                jewel_size: None,
            },

            SpreadUp => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            ReloadSpeed => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            ThunderAlignment => SkillDesc {
                limit: 5,
                jewel_size: None,
            },

            Guard => SkillDesc {
                limit: 5,
                jewel_size: Some(2),
            },

            StaminaSurge => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Earplugs => SkillDesc {
                limit: 5,
                jewel_size: Some(3),
            },

            BowChargePlus => SkillDesc {
                limit: 1,
                jewel_size: None,
            },

            BlastResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            AmmoUp => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },

            LeapofFaith => SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },

            DragonResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            WaterResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            DivineBlessing => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            RecoverySpeed => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            SpeedSharpening => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            MuckResistance => SkillDesc {
                limit: 2,
                jewel_size: Some(1),
            },

            PowerProlonger => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            TremorResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            HellfireCloak => SkillDesc {
                limit: 4,
                jewel_size: Some(3),
            },

            BubblyDance => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            PunishingDraw => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            WallRunner => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            GuardUp => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            CriticalBoost => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            MindsEye => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            BlastAttack => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            MasterMounter => SkillDesc {
                limit: 1,
                jewel_size: Some(2),
            },

            Counterstrike => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            ThunderAttack => SkillDesc {
                limit: 5,
                jewel_size: Some(1),
            },

            Artillery => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },

            Bombardier => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            CaptureMaster => SkillDesc {
                limit: 1,
                jewel_size: None,
            },

            Diversion => SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },

            FireResistance => SkillDesc {
                limit: 3,
                jewel_size: Some(1),
            },

            HornMaestro => SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
            Ballistics => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
            KushalaBlessing => SkillDesc {
                limit: 4,
                jewel_size: None,
            },
            ChameleosBlessing => SkillDesc {
                limit: 4,
                jewel_size: None,
            },
            TeostraBlessing => SkillDesc {
                limit: 4,
                jewel_size: None,
            },
            MastersTouch => SkillDesc {
                limit: 3,
                jewel_size: Some(2),
            },
            RapidFireUp => SkillDesc {
                limit: 3,
                jewel_size: Some(3),
            },
            CarvingPro => SkillDesc {
                limit: 1,
                jewel_size: Some(1),
            },
        }
    }
}
