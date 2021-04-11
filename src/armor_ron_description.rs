use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    HornMaestro
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