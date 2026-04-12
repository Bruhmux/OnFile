use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "category", rename_all = "lowercase")]
pub enum Category {
    Suspect,
    Weapon,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Clue {
    pub id: Uuid,
    pub room_id: String,
    pub x_category: Category,
    pub x_idx: i32,
    pub y_category: Category,
    pub y_idx: i32,
    pub is_true: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discovery {
    pub id: Uuid,
    pub player_id: Uuid,
    pub room_id: String,
    pub clue_id: Uuid,
}

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

#[derive(Serialize)]
struct File {
    suspect: Suspect,
    weapon: Weapon,
    location: Location,
    verdict: Verdict,
}

trait Evidence {
    fn index(&self) -> usize;
    fn category(&self) -> &'static str;
}

trait Coordinate {}

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

impl Evidence for Suspect {
    fn category(&self) -> &'static str {
        "Suspect"
    }
    fn index(&self) -> usize {
        *self as usize
    }
}

impl Evidence for Weapon {
    fn category(&self) -> &'static str {
        "Weapon"
    }
    fn index(&self) -> usize {
        *self as usize
    }
}

impl Evidence for Location {
    fn category(&self) -> &'static str {
        "Location"
    }
    fn index(&self) -> usize {
        *self as usize
    }
}
