use rocket_okapi::openapi_get_routes;

mod home;
mod hub;
mod user;

pub fn routes() -> std::vec::Vec<rocket::Route> {
    use home::*;
    use hub::*;
    use user::*;
    openapi_get_routes![
        homepage,
        turn,
        reset,
        register,
        login,
        check_logged_in,
        get_games,
        create_game,
        get_game_by_id,
        game_register,
        game_turn,
        game_reset,
        game_delete
    ]
}
