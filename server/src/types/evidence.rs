use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Suspect {
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
pub enum Weapon {
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
pub enum Location {
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
pub enum Verdict {
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

impl From<bool> for Verdict {
    fn from(v: bool) -> Self {
        match v {
            true => Self::Guilty,
            false => Self::Innocent,
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
