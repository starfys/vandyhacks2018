#![feature(proc_macro_hygiene, decl_macro)]

extern crate bcrypt;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hex;
extern crate rand;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use dotenv::dotenv;

use routes::*;

use std::env;
use std::io;

// TODO: add diesel
// TODO: postgres integration

fn main() -> Result<(), io::Error> {
    // Load environment from .env
    dotenv().map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    // Get db from environment
    let database_url =
        env::var("DATABASE_URL").map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    // Start the web server
    rocket::ignite()
        .manage(db::connect(&database_url))
        .mount(
            "/api",
            routes![
                add_task,
                remove_task,
                modify_task,
                list_tasks,
                start_work,
                finish_work,
                list_work,
                add_user,
                remove_user,
                modify_user
            ],
        )
        .launch();
    Ok(())
}
