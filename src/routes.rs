use chrono::{Days, Utc};
use rocket::{get, post, serde::json::Json, tokio::sync::Mutex, State};
use rocket_okapi::{okapi::schemars::JsonSchema, openapi, openapi_get_routes};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{encode_token, Claims, Secret},
    tictac::TicTacToeGame,
};

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

#[derive(Deserialize, JsonSchema)]
pub struct UserData {
    username: String,
    password: String,
}

#[derive(Serialize, JsonSchema)]
pub struct RegistrationResult {
    id: i32,
    username: String,
}

#[openapi(tag = "User control")]
#[post("/user/register", data = "<data>")]
pub async fn register(
    db: &State<DatabaseConnection>,
    data: Json<UserData>,
) -> Option<Json<RegistrationResult>> {
    use crate::database::user;
    let new_user = user::ActiveModel {
        username: Set(data.username.clone()),
        password: Set(data.password.clone()),
        ..Default::default()
    };
    let user = new_user.insert(&**db).await.ok()?;
    let returned = RegistrationResult {
        id: user.id,
        username: user.username,
    };
    Some(Json(returned))
}

#[openapi(tag = "User control")]
#[post("/user/login", data = "<data>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    data: Json<UserData>,
    secret: &State<Secret>,
) -> Json<Option<String>> {
    use crate::database::user;
    let x = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Username.eq(data.username.clone()))
                .add(user::Column::Password.eq(data.password.clone())),
        )
        .one(&**db)
        .await;

    let timestamp = Utc::now()
        .checked_add_days(Days::new(1))
        .unwrap()
        .timestamp();
    Json(if let Ok(Some(_user)) = x {
        let claims = Claims {
            username: _user.username,
            exp: timestamp,
        };
        encode_token(&claims, &secret.0)
    } else {
        None
    })
}

#[openapi(tag = "User control")]
#[post("/user/check")]
pub async fn check_logged_in(claims: Option<Claims>) -> Json<Option<String>> {
    Json(claims.map(|x| x.username))
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    openapi_get_routes![homepage, turn, reset, register, login, check_logged_in]
}
