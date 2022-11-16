use rocket::{get, post, serde::json::Json, tokio::sync::Mutex, State};
use rocket_okapi::{okapi::schemars::JsonSchema, openapi, openapi_get_routes};
use serde::Deserialize;

use crate::tictac::TicTacToeGame;

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[get("/")]
pub async fn homepage(game: &State<Mutex<TicTacToeGame>>) -> Json<TicTacToeGame> {
    let game = game.lock().await;
    Json((*game).clone())
}

#[derive(Deserialize, JsonSchema)]
pub struct TurnCommand {
    x: usize,
    y: usize,
}

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[post("/turn", data = "<cmd>")]
pub async fn turn(
    game: &State<Mutex<TicTacToeGame>>,
    cmd: Json<TurnCommand>,
) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    (*game).turn(cmd.x, cmd.y);
    Json((*game).clone())
}

#[derive(Deserialize, JsonSchema)]
pub struct ResetCommand {
    size: usize,
    criteria: usize,
}

#[openapi(tag = "Homepage Tic-tac-toc game")]
#[post("/reset", data = "<cmd>")]
pub async fn reset(
    game: &State<Mutex<TicTacToeGame>>,
    cmd: Json<ResetCommand>,
) -> Json<TicTacToeGame> {
    let mut game = game.lock().await;
    *game = TicTacToeGame::new(cmd.size, cmd.criteria);
    Json((*game).clone())
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    openapi_get_routes![homepage, turn, reset]
}
