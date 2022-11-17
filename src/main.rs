#![deny(warnings)]

use rocket::{launch, tokio::sync::Mutex};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sea_orm::Database;

pub mod database;
mod hub;
mod routes;
mod tictac;

#[launch]
async fn rocket() -> _ {
    let db = Database::connect("sqlite://root.db").await.unwrap();

    let allowed_origins = AllowedOrigins::All;
    let cors = CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .manage(Mutex::new(tictac::TicTacToeGame::new(3, 3)))
        .manage(db)
        .mount("/", routes::routes())
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .attach(cors)
}
