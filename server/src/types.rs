use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
    friends: Vec<u64>,
}

#[derive(Serialize)]
pub struct Room {
    id: [u8; 5],
    players: Vec<User>,
    files: Vec<File>,
    grid: LogicGrid,
}

#[derive(Serialize)]
struct LogicGrid {
    suspect_weapon: [[Option<bool>; 8]; 8],
    suspect_location: [[Option<bool>; 8]; 8],
    weapon_location: [[Option<bool>; 8]; 8],
}

impl LogicGrid {
    pub fn new() -> Self {
        Self {
            suspect_weapon: [[None; 8]; 8],
            suspect_location: [[None; 8]; 8],
            weapon_location: [[None; 8]; 8],
        }
    }

    pub fn add_evidence<T, U>(&mut self, item1: T, item2: U, is_valid: bool)
    where
        T: Evidence,
        U: Evidence,
    {
        let i = item1.index();
        let j = item2.index();

        match (item1.category(), item2.category()) {
            ("Suspect", "Weapon") => self.suspect_weapon[i][j] = Some(is_valid),
            ("Weapon", "Suspect") => self.suspect_weapon[j][i] = Some(is_valid),

            ("Suspect", "Location") => self.suspect_location[i][j] = Some(is_valid),
            ("Location", "Suspect") => self.suspect_location[j][i] = Some(is_valid),

            ("Weapon", "Location") => self.weapon_location[i][j] = Some(is_valid),
            ("Location", "Weapon") => self.weapon_location[j][i] = Some(is_valid),

            _ => println!("Warning: Attempted to compare the same category or unsupported types."),
        }
    }
}

#[derive(Serialize)]
struct File {
    suspect: Suspect,
    weapon: Weapon,
    location: Location,
    verdict: Verdict,
}

trait Evidence {
    fn index(&self) -> u8;
    fn category(&self) -> &'static str;
}

#[derive(PartialEq)]
enum EvidenceType {
    Suspect,
    Weapon,
    Location,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Suspect {
    TavernkeepGarrick,
    KnightRowan,
    WizardBjorn,
    ThiefJax,
    PriestThalos,
    AlchemistNox,
    LibrarianMildra,
    MaidAnya,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Weapon {
    WoodenStake,
    BloodyDagger,
    PoisonVial,
    RustySword,
    StainedGrimoir,
    SketchyPortal,
    StrangeOoze,
    FlacidWand,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Location {
    GuildHall,
    AlchemyLab,
    ThroneRoom,
    TipsyTavern,
    AncientLibrary,
    MysticArcanum,
    RoyalStables,
    MustyCellar,
}

#[derive(Debug, Serialize, Deserialize)]
enum Verdict {
    Guilty,
    Innocent,
}

impl Display for Suspect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Suspect::TavernkeepGarrick => f.write_str("Tavernkeep Garrick"),
            Suspect::KnightRowan => f.write_str("KnightRowan"),
            Suspect::WizardBjorn => f.write_str("WizardBjorn"),
            Suspect::ThiefJax => f.write_str("ThiefJax"),
            Suspect::PriestThalos => f.write_str("PriestThalos"),
            Suspect::AlchemistNox => f.write_str("AlchemistNox"),
            Suspect::LibrarianMildra => f.write_str("LibrarianMildra"),
            Suspect::MaidAnya => f.write_str("MaidAnya"),
        }
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Weapon::WoodenStake => f.write_str("Wooden Stake"),
            Weapon::BloodyDagger => f.write_str("Bloody Dagger"),
            Weapon::PoisonVial => f.write_str("Poison Vial"),
            Weapon::RustySword => f.write_str("Rusty Sword"),
            Weapon::StainedGrimoir => f.write_str("Stained Grimoir"),
            Weapon::SketchyPortal => f.write_str("Sketchy Portal"),
            Weapon::StrangeOoze => f.write_str("Strange Ooze"),
            Weapon::FlacidWand => f.write_str("Flacid Wand"),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Location::GuildHall => f.write_str("Guild Hall"),
            Location::AlchemyLab => f.write_str("Alchemy Lab"),
            Location::ThroneRoom => f.write_str("Throne Room"),
            Location::TipsyTavern => f.write_str("Tipsy Tavern"),
            Location::AncientLibrary => f.write_str("Ancient Library"),
            Location::MysticArcanum => f.write_str("Mystic Arcanum"),
            Location::RoyalStables => f.write_str("Royal Stables"),
            Location::MustyCellar => f.write_str("Musty Cellar"),
        }
    }
}

impl Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Verdict::Guilty => f.write_str("Guilty"),
            Verdict::Innocent => f.write_str("Innocent"),
        }
    }
}

impl Evidence for Suspect {
    fn category(&self) -> &'static str {
        "Suspect"
    }
    fn index(&self) -> u8 {
        *self as u8
    }
}

impl Evidence for Weapon {
    fn category(&self) -> &'static str {
        "Weapon"
    }
    fn index(&self) -> u8 {
        *self as u8
    }
}

impl Evidence for Location {
    fn category(&self) -> &'static str {
        "Location"
    }
    fn index(&self) -> u8 {
        *self as u8
    }
}
