use rocket::{get, post, State, tokio::sync::Mutex, serde::json::Json};
use serde::Deserialize;

use crate::tictac::TicTacToeGame;

#[get("/")]
pub async fn homepage(game: &State<Mutex<TicTacToeGame>>) -> Json<TicTacToeGame> {
    let game = game.lock().await;
    Json((*game).clone())
}

#[derive(Deserialize)]
pub struct TurnCommand {
    x: usize,
    y: usize,
}

#[post("/turn", data="<cmd>")]
pub async fn turn(game: &State<Mutex<TicTacToeGame>>, cmd: Json<TurnCommand>) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    (*game).turn(cmd.x, cmd.y);
    Json((*game).clone())
}

#[derive(Deserialize)]
pub struct ResetCommand {
    size: usize,
    criteria: usize,
}

#[post("/reset", data="<cmd>")]
pub async fn reset(game: &State<Mutex<TicTacToeGame>>, cmd: Json<ResetCommand>) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    *game = TicTacToeGame::new(cmd.size, cmd.criteria);
    Json((*game).clone())
}