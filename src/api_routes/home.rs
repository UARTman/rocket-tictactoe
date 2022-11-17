use rocket::{get, post, serde::json::Json, tokio::sync::Mutex, State};
use rocket_okapi::{okapi::schemars::JsonSchema, openapi};
use serde::Deserialize;

use crate::tictac::TicTacToeGame;

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[get("/")]
pub async fn homepage(game: &State<Mutex<TicTacToeGame>>) -> Json<TicTacToeGame> {
    let game = game.lock().await;
    Json((*game).clone())
}

#[derive(Deserialize, JsonSchema)]
pub struct TurnData {
    pub x: usize,
    pub y: usize,
}

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[post("/turn", data = "<data>")]
pub async fn turn(game: &State<Mutex<TicTacToeGame>>, data: Json<TurnData>) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    (*game).turn(data.x, data.y);
    Json((*game).clone())
}

#[derive(Deserialize, JsonSchema)]
pub struct ResetData {
    pub size: usize,
    pub criteria: usize,
}

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[post("/reset", data = "<data>")]
pub async fn reset(
    game: &State<Mutex<TicTacToeGame>>,
    data: Json<ResetData>,
) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    *game = TicTacToeGame::new(data.size, data.criteria);
    Json((*game).clone())
}
