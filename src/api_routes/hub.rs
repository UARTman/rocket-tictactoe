use crate::{
    auth::Claims,
    hub::{Hub, HubEntry}, tictac::{Cell, TicTacToeGame},
};
use rocket::{get, post, serde::json::Json, tokio::sync::RwLock, State, delete};
use rocket_okapi::{openapi, JsonSchema};
use serde::Deserialize;
use std::collections::HashMap;

use super::home::{ResetData, TurnData};

#[openapi(tag = "Games")]
#[get("/games")]
pub async fn get_games(hub: &State<RwLock<Hub>>) -> Json<HashMap<u64, HubEntry>> {
    let mut x = HashMap::new();
    for (k, l) in &hub.read().await.entries {
        let _l = l.read().await;
        x.insert(*k, _l.clone());
    }

    Json(x)
}

#[openapi(tag = "Games")]
#[post("/games", data = "<data>")]
pub async fn create_game(
    hub: &State<RwLock<Hub>>,
    _claims: Claims,
    data: Json<ResetData>,
) -> Json<Option<u64>> {
    Json(hub.write().await.new_game(data.0.size, data.0.criteria))
}

#[openapi(tag = "Games")]
#[get("/games/<id>")]
pub async fn get_game_by_id(hub: &State<RwLock<Hub>>, id: u64) -> Option<Json<HubEntry>> {
    Some(Json(
        hub.read().await.entries.get(&id)?.read().await.clone(),
    ))
}

#[derive(Deserialize, JsonSchema)]
pub struct RegisterData {
    #[serde(rename="as")]
    _as: Cell,
}

#[openapi(tag = "Games")]
#[post("/games/<id>/register", data="<data>")]
pub async fn game_register(hub: &State<RwLock<Hub>>, id: u64, claims: Claims, data: Json<RegisterData>) -> Option<Json<HubEntry>> {
    let hub = hub.read().await;
    let mut game = hub.entries.get(&id)?.write().await;
    game.set_player(data.0._as, claims.username);
    Some(Json(game.clone()))
}

#[openapi(tag = "Games")]
#[post("/games/<id>/turn", data="<data>")]
pub async fn game_turn(hub: &State<RwLock<Hub>>, id: u64, claims: Claims, data: Json<TurnData>) -> Option<Json<HubEntry>> {
    let hub = hub.read().await;
    let mut game = hub.entries.get(&id)?.write().await;
    game.turn(data.0.x, data.0.y, claims.username);
    Some(Json(game.clone()))
}

#[openapi(tag = "Games")]
#[post("/games/<id>/reset", data="<data>")]
pub async fn game_reset(hub: &State<RwLock<Hub>>, id: u64, claims: Claims, data: Json<ResetData>) -> Option<Json<HubEntry>> {
    let hub = hub.read().await;
    let mut game = hub.entries.get(&id)?.write().await;
    if game.is_a_player(claims.username) {
        game.game = TicTacToeGame::new(data.0.size, data.0.criteria);
    }
    Some(Json(game.clone()))
}

#[openapi(tag = "Games")]
#[delete("/games/<id>")]
pub async fn game_delete(hub: &State<RwLock<Hub>>, id: u64, claims: Claims) -> Option<Json<bool>> {
    let mut hub = hub.write().await;
    let allowed = {
        let game = hub.entries.get(&id)?.read().await;
        game.is_a_player(claims.username) && game.game.draw
    };
    if allowed {
        hub.entries.remove(&id);
        Some(Json(true))
    } else {
        Some(Json(false))
    }
}