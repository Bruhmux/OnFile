use crate::db::tables::{Category, Clue};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize)]
pub struct LogicGrid {
    pub suspect_weapon: [[Option<bool>; 8]; 8],
    pub suspect_location: [[Option<bool>; 8]; 8],
    pub weapon_location: [[Option<bool>; 8]; 8],
}

impl LogicGrid {
    pub fn new() -> Self {
        Self {
            suspect_weapon: [[None; 8]; 8],
            suspect_location: [[None; 8]; 8],
            weapon_location: [[None; 8]; 8],
        }
    }

    pub fn from_clues(clues: Vec<Clue>) -> Self {
        let mut grid = Self::new();
        for clue in clues {
            grid.apply_clue(&clue);
        }
        grid
    }

    fn apply_clue(&mut self, clue: &Clue) {
        let i = clue.x_idx as usize;
        let j = clue.y_idx as usize;
        let val = Some(clue.is_true);

        match (&clue.x_category, &clue.y_category) {
            (Category::Suspect, Category::Weapon) => self.suspect_weapon[i][j] = val,
            (Category::Weapon, Category::Suspect) => self.suspect_weapon[j][i] = val,

            (Category::Suspect, Category::Location) => self.suspect_location[i][j] = val,
            (Category::Location, Category::Suspect) => self.suspect_location[j][i] = val,

            (Category::Weapon, Category::Location) => self.weapon_location[i][j] = val,
            (Category::Location, Category::Weapon) => self.weapon_location[j][i] = val,
            _ => {}
        }
    }
}

impl Default for LogicGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize)]
struct File {
    suspect: Suspect,
    weapon: Weapon,
    location: Location,
    verdict: Verdict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum Verdict {
    Guilty,
    Innocent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Evidence {
    Suspect(Suspect),
    Weapon(Weapon),
    Location(Location),
}

impl Evidence {
    pub fn index(self) -> u8 {
        match self {
            Self::Suspect(s) => s as u8,
            Self::Weapon(w) => w as u8,
            Self::Location(l) => l as u8,
        }
    }
    pub fn type_str(self) -> &'static str {
        match self {
            Self::Suspect(_) => "suspect",
            Self::Weapon(_) => "weapon",
            Self::Location(_) => "location",
        }
    }
}

impl TryFrom<u8> for Suspect {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::TavernkeepGarrick),
            1 => Ok(Self::KnightRowan),
            2 => Ok(Self::WizardBjorn),
            3 => Ok(Self::ThiefJax),
            4 => Ok(Self::PriestThalos),
            5 => Ok(Self::AlchemistNox),
            6 => Ok(Self::LibrarianMildra),
            7 => Ok(Self::MaidAnya),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Weapon {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::WoodenStake),
            1 => Ok(Self::BloodyDagger),
            2 => Ok(Self::PoisonVial),
            3 => Ok(Self::RustySword),
            4 => Ok(Self::StainedGrimoir),
            5 => Ok(Self::SketchyPortal),
            6 => Ok(Self::StrangeOoze),
            7 => Ok(Self::FlacidWand),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Location {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::GuildHall),
            1 => Ok(Self::AlchemyLab),
            2 => Ok(Self::ThroneRoom),
            3 => Ok(Self::TipsyTavern),
            4 => Ok(Self::AncientLibrary),
            5 => Ok(Self::MysticArcanum),
            6 => Ok(Self::RoyalStables),
            7 => Ok(Self::MustyCellar),
            _ => Err(()),
        }
    }
}

impl Display for Suspect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Suspect::TavernkeepGarrick => f.write_str("Tavernkeep Garrick"),
            Suspect::KnightRowan => f.write_str("Knight Rowan"),
            Suspect::WizardBjorn => f.write_str("Wizard Bjorn"),
            Suspect::ThiefJax => f.write_str("Thief Jax"),
            Suspect::PriestThalos => f.write_str("Priest Thalos"),
            Suspect::AlchemistNox => f.write_str("Alchemist Nox"),
            Suspect::LibrarianMildra => f.write_str("Librarian Mildra"),
            Suspect::MaidAnya => f.write_str("Maid Anya"),
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
