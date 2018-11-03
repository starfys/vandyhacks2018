#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db;
mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::json::Json;

use models::{Event, User};

use std::env;
use std::io;

// TODO: add diesel
// TODO: postgres integration

/// Adds an event
// TODO: require cookie auth
#[post("/event/add", format = "application/json", data = "<event>")]
fn add_event(event: Json<Event>, db: db::DbConn) -> Result<&'static str, io::Error> {
    // Insert event into database
    diesel::insert_into(schema::events::table)
        .values(&*event)
        .get_result::<Event>(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Removes an event
// TODO: require cookie auth
#[post("/event/remove/<event_id>")]
fn remove_event(event_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    use schema::events::dsl;
    // Remove user from database
    diesel::delete(dsl::events.filter(dsl::event_id.eq(event_id)))
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Modifies an event
// TODO: require cookie auth
#[post(
    "/event/modify/<event_id>",
    format = "application/json",
    data = "<event>"
)]
fn modify_event(
    event_id: i64,
    event: Json<Event>,
    db: db::DbConn,
) -> Result<&'static str, io::Error> {
    // TODO: implement
    Ok("")
}

/// Lists events
// TODO: require cookie auth
#[get("/event/list/<user_id>")]
fn list_events(user_id: i64, db: db::DbConn) -> Result<Json<Vec<Event>>, io::Error> {
    use schema::events::dsl;
    dsl::events
        .filter(dsl::user_id.eq(user_id))
        .load::<Event>(&*db)
        .map(|events| Json(events))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Adds a user
// TODO: require cookie auth
#[post("/user/add", format = "application/json", data = "<user>")]
fn add_user(user: Json<User>, db: db::DbConn) -> Result<&'static str, io::Error> {
    // Insert user into database
    diesel::insert_into(schema::users::table)
        .values(&*user)
        .get_result::<User>(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

// TODO: require cookie auth
#[post("/user/modify/<user_id>", format = "application/json", data = "<user>")]
fn modify_user(user_id: i64, user: Json<User>, db: db::DbConn) -> Result<&'static str, io::Error> {
    Ok("")
}
/// Removes a user
// TODO: require cookie auth
#[post("/user/remove/<user_id>")]
fn remove_user(user_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    use schema::users::dsl;
    // Remove user from database
    diesel::delete(dsl::users.filter(dsl::user_id.eq(user_id)))
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

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
                add_event,
                remove_event,
                modify_event,
                list_events,
                add_user,
                remove_user,
                modify_user
            ],
        )
        .launch();
    Ok(())
}
