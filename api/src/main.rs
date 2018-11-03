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

use models::{Task, TaskInsert, User, WorkFinish, WorkStart};

use std::env;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: add diesel
// TODO: postgres integration

/// Adds an task
// TODO: require cookie auth
#[post("/task/add", format = "application/json", data = "<task>")]
fn add_task(task: Json<TaskInsert>, db: db::DbConn) -> Result<&'static str, io::Error> {
    // Insert task into database
    diesel::insert_into(schema::tasks::table)
        .values(&*task)
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Removes an task
// TODO: require cookie auth
#[post("/task/remove/<task_id>")]
fn remove_task(task_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    use schema::tasks::dsl;
    // Remove user from database
    diesel::delete(dsl::tasks.filter(dsl::task_id.eq(task_id)))
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Modifies an task
// TODO: require cookie auth
#[post("/task/modify/<task_id>", format = "application/json", data = "<task>")]
fn modify_task(task_id: i64, task: Json<Task>, db: db::DbConn) -> Result<&'static str, io::Error> {
    // TODO: implement
    Ok("")
}

/// Lists tasks
// TODO: require cookie auth
#[get("/task/list/<user_id>")]
fn list_tasks(user_id: i64, db: db::DbConn) -> Result<Json<Vec<Task>>, io::Error> {
    use schema::tasks::dsl;
    dsl::tasks
        // Get all tasks for which the owner is the user
        .filter(dsl::owner_id.eq(user_id))
        // Get all tasks that are not completed
        .filter(dsl::completed.eq(false))
        // Sort by due date
        .order(dsl::due.asc())
        // Execute on database and return tasks as struct
        .load::<Task>(&*db)
        // Convert to json
        .map(|tasks| Json(tasks))
        // Convert error to io::Error
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Starts work
// TODO: require cookie auth
#[post("/work/start/<task_id>")]
fn start_work(task_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    // Get current time
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    // Convert to microseconds
    let time = time.as_secs() * 1_000_000 + u64::from(time.subsec_micros());
    // Create a work starter
    let work_start = WorkStart {
        task_id,
        start_time: time as i64,
        end_time: -1,
    };
    // Insert task into database
    diesel::insert_into(schema::work::table)
        .values(&work_start)
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Finishes work
// TODO: require cookie auth
#[post(
    "/work/finish/<task_id>",
    format = "application/json",
    data = "<finish_data>"
)]
fn finish_work(
    task_id: i64,
    finish_data: Json<WorkFinish>,
    db: db::DbConn,
) -> Result<&'static str, io::Error> {
    use schema::work::dsl;
    // Extract from json
    // Add the time into the data
    let end_time = finish_data.end_time.unwrap_or_else(|| {
        // Get current time
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .map(|time| time.as_secs() * 1_000_000 + u64::from(time.subsec_micros()))
            .unwrap_or(0) as i64
    });
    // Insert task into database
    diesel::update(
        dsl::work
            .filter(dsl::task_id.eq(task_id))
            .filter(dsl::end_time.eq(-1)),
    )
    .set((
        dsl::end_time.eq(end_time),
        dsl::progress.eq(finish_data.progress),
        dsl::finished.eq(finish_data.finished),
        dsl::music.eq(finish_data.music),
        dsl::interruptions.eq(finish_data.interruptions),
        dsl::noise.eq(finish_data.noise),
        dsl::meetings.eq(finish_data.meetings),
        dsl::breaks.eq(finish_data.breaks),
    ))
    .execute(&*db)
    .map(|_| "")
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
                add_task,
                remove_task,
                modify_task,
                list_tasks,
                start_work,
                finish_work,
                add_user,
                remove_user,
                modify_user
            ],
        )
        .launch();
    Ok(())
}
