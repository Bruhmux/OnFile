use crate::{
    db::tables::{Category, Clue},
    types::discovery::{Discovery, init_deck},
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LogicGrid {
    pub suspect_weapon: Vec<Vec<Option<bool>>>,
    pub suspect_location: Vec<Vec<Option<bool>>>,
    pub weapon_location: Vec<Vec<Option<bool>>>,
}

impl LogicGrid {
    pub fn new(amount: usize) -> Self {
        Self {
            suspect_weapon: vec![vec![None; amount]; amount],
            suspect_location: vec![vec![None; amount]; amount],
            weapon_location: vec![vec![None; amount]; amount],
        }
    }

    pub fn from_clues(amount: usize, clues: Vec<Clue>) -> Self {
        let mut grid = Self::new(amount);
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

#[derive(Serialize, Deserialize, Default)]
pub struct Deck {
    cards: Vec<Discovery>,
}

impl Deck {
    pub fn init_deck() -> Vec<Discovery> {
        let mut wild_cards = vec![Discovery::Wild; 2];
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

    pub fn draw(&mut self) -> Discovery {
        if self.cards.is_empty() {
            self.cards.append(&mut init_deck());
        }
        self.cards.pop().unwrap()
    }
}
