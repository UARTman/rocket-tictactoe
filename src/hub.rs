use std::collections::HashMap;

use rocket::tokio::sync::RwLock;
use rocket_okapi::JsonSchema;
use serde::Serialize;

use crate::tictac::{Cell, TicTacToeGame};

pub struct Hub {
    pub entries: HashMap<u64, RwLock<HubEntry>>,
    id_counter: u64,
}

impl Hub {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn new_game(&mut self, side: usize, criteria: usize) -> Option<u64> {
        if side == 0 || side > 100 || criteria > side {
            return None;
        }
        self.entries.insert(
            self.id_counter,
            RwLock::new(HubEntry {
                game: TicTacToeGame::new(side, criteria),
                x_player: None,
                o_player: None,
            }),
        );
        self.id_counter += 1;
        Some(self.id_counter - 1)
    }
}

#[derive(Serialize, Clone, JsonSchema)]
pub struct HubEntry {
    pub game: TicTacToeGame,
    pub x_player: Option<String>,
    pub o_player: Option<String>,
}

impl HubEntry {
    fn get_player(&self, kind: Cell) -> Option<String> {
        match kind {
            Cell::X => self.x_player.clone(),
            Cell::O => self.o_player.clone(),
        }
    }

    pub fn set_player(&mut self, kind: Cell, player: String) {
        match kind {
            Cell::X => if self.x_player.is_none() {self.x_player = Some(player)},
            Cell::O => if self.o_player.is_none() {self.o_player = Some(player)},
        }
    }

    pub fn is_a_player(&self, player: String) -> bool {
        if let Some(p) = self.x_player.clone() {
            p == player
        } else if let Some(p) = self.o_player.clone()  {
            p == player
        } else {
            false
        }
    }

    pub fn turn(&mut self, x: usize, y: usize, player_name: String) {
        let next = self.game.current_player;
        if next.is_none() {
            return;
        }
        let next = next.unwrap();
        let supposed_player = self.get_player(next);
        if supposed_player.is_none() || supposed_player.unwrap() != player_name {
            return;
        }
        self.game.turn(x, y);
    }
}
