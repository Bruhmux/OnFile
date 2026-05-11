use crate::{
    db::tables::Category,
    types::evidence::{Location, Suspect, Verdict, Weapon},
};
use rand::{RngExt, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Discovery {
    Wild,
    Same(Category),
    Different(Category, Category),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    suspect: Suspect,
    weapon: Weapon,
    location: Location,
    verdict: Verdict,
}

impl File {
    pub fn suspect(&self) -> Suspect {
        self.suspect
    }

    pub fn weapon(&self) -> Weapon {
        self.weapon
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn verdict(&self) -> Verdict {
        self.verdict
    }
}

pub fn init_files(amount: u8) -> Vec<File> {
    let mut rng = rand::rng();

    let mut suspect_list: Vec<u8> = (0..amount).collect();
    let mut location_list: Vec<u8> = (0..amount).collect();
    let mut weapon_list: Vec<u8> = (0..amount).collect();
    let guilty_index = rng.random_range(0..amount);

    suspect_list.shuffle(&mut rng);
    location_list.shuffle(&mut rng);
    weapon_list.shuffle(&mut rng);

    (0..amount as usize)
        .map(|i| File {
            suspect: Suspect::try_from(suspect_list[i]).unwrap(),
            weapon: Weapon::try_from(weapon_list[i]).unwrap(),
            location: Location::try_from(location_list[i]).unwrap(),
            verdict: Verdict::from(i as u8 == guilty_index),
        })
        .collect()
}

pub fn init_deck() -> Vec<Discovery> {
    let mut wild_cards = vec![Discovery::Wild; 6];
    let mut suspect_cards = vec![Discovery::Same(Category::Suspect); 2];
    let mut location_cards = vec![Discovery::Same(Category::Location); 2];
    let mut weapon_cards = vec![Discovery::Same(Category::Weapon); 2];
    let mut suspect_location_cards =
        vec![Discovery::Different(Category::Suspect, Category::Location); 2];
    let mut suspect_weapon_cards =
        vec![Discovery::Different(Category::Suspect, Category::Weapon); 2];
    let mut location_weapon_cards =
        vec![Discovery::Different(Category::Location, Category::Weapon); 2];

    let mut deck = vec![];
    deck.append(&mut wild_cards);
    deck.append(&mut suspect_cards);
    deck.append(&mut location_cards);
    deck.append(&mut weapon_cards);
    deck.append(&mut suspect_location_cards);
    deck.append(&mut suspect_weapon_cards);
    deck.append(&mut location_weapon_cards);
    deck.shuffle(&mut rand::rng());
    deck
}
