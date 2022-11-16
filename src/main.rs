#![deny(warnings)]

use rocket::{launch, routes, tokio::sync::Mutex};
use rocket_cors::{AllowedOrigins, CorsOptions};

mod hub;
mod tictac;
mod routes;

use routes::{homepage, reset, turn};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::All;
    let cors = CorsOptions {
        allowed_origins,
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .manage(Mutex::new(tictac::TicTacToeGame::new(3, 3)))
        .mount("/", routes![homepage, turn, reset])
        .attach(cors)
}
