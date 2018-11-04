use bcrypt;
use diesel::prelude::*;
use hex;
use rand::Rng;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use rocket_contrib::json::Json;

use db;
use models::*;
use schema;

use std::collections::HashMap;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

/// Adds an task
// TODO: require cookie auth
#[post("/user/<user_id>/task", format = "application/json", data = "<task>")]
pub fn add_task(
    user_id: i64,
    task: Json<TaskInsert>,
    db: db::DbConn,
) -> Result<Json<Task>, io::Error> {
    // Insert task into database
    diesel::insert_into(schema::tasks::table)
        .values(&*task)
        .get_result::<Task>(&*db)
        .map(|task| Json(task))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Removes an task
// TODO: require cookie auth
#[delete("/user/<user_id>/task/<task_id>")]
pub fn remove_task(user_id: i64, task_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    use schema::tasks::dsl;
    // Remove user from database
    diesel::delete(dsl::tasks.filter(dsl::task_id.eq(task_id)))
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Removes an task
// TODO: require cookie auth
#[get("/user/<user_id>/task/<task_id>")]
pub fn get_task(user_id: i64, task_id: i64, db: db::DbConn) -> Result<Json<Task>, io::Error> {
    use schema::tasks::dsl;
    // Remove user from database
    dsl::tasks
        .filter(dsl::owner_id.eq(user_id))
        .filter(dsl::task_id.eq(task_id))
        .limit(1)
        .load::<Task>(&*db)
        // Convert the error to io::Error
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        // Only take the first task
        .and_then(|task| {
            task.into_iter()
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to find matching task"))
        })
        // Convert to json
        .map(|task| Json(task))
        // Fix error
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Modifies an task
// TODO: require cookie auth
#[put(
    "/user/<user_id>/task/<task_id>",
    format = "application/json",
    data = "<task>"
)]
pub fn modify_task(
    user_id: i64,
    task_id: i64,
    task: Json<Task>,
    db: db::DbConn,
) -> Result<&'static str, io::Error> {
    // TODO: implement
    Ok("")
}

/// Lists tasks
// TODO: require cookie auth
#[get("/user/<user_id>/task")]
pub fn list_tasks(user_id: i64, db: db::DbConn) -> Result<Json<Vec<Task>>, io::Error> {
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
#[post("/user/<user_id>/task/<task_id>/start")]
pub fn start_work(user_id: i64, task_id: i64, db: db::DbConn) -> Result<Json<Work>, io::Error> {
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
        .get_result::<Work>(&*db)
        .map(|work| Json(work))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Finishes work
// TODO: require cookie auth
#[post(
    "/user/<user_id>/task/<task_id>/finish",
    format = "application/json",
    data = "<finish_data>"
)]
pub fn finish_work(
    user_id: i64,
    task_id: i64,
    finish_data: Json<WorkFinish>,
    db: db::DbConn,
) -> Result<Json<Work>, io::Error> {
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
    .get_result::<Work>(&*db)
    .map(|work| {
        // Pass data to ms upload daemon
        let client = reqwest::Client::new();
        client.post("http://localhost:8080/work").json(&work).send();
        Json(work)
    })
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Manually inserts work
// TODO: require cookie auth
#[post(
    "/user/<user_id>/task/<task_id>/work",
    format = "application/json",
    data = "<work>"
)]
pub fn add_work(
    user_id: i64,
    task_id: i64,
    work: Json<WorkInsert>,
    db: db::DbConn,
) -> Result<Json<Work>, io::Error> {
    use schema::work::dsl;
    // Insert task into database
    diesel::insert_into(dsl::work)
        .values(&*work)
        .get_result::<Work>(&*db)
        .map(|work| {
            // Pass data to ms upload daemon
            let client = reqwest::Client::new();
            client.post("http://localhost:8080/work").json(&work).send();
            // Convert data to json for returning
            Json(work)
        })
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Dumps the work database
// TODO: require cookie auth
#[get("/work")]
pub fn list_work(db: db::DbConn) -> Result<Json<Vec<Work>>, io::Error> {
    use schema::work::dsl;
    dsl::work
        // Execute on database and return tasks as struct
        .load::<Work>(&*db)
        // Convert to json
        .map(|tasks| Json(tasks))
        // Convert error to io::Error
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Adds a user
// TODO: require cookie auth
#[post("/user", format = "application/json", data = "<user>")]
pub fn add_user(mut user: Json<User>, db: db::DbConn) -> Result<&'static str, io::Error> {
    // Replace the user's password with a hashed version
    user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    // Insert user into database
    diesel::insert_into(schema::users::table)
        .values(&*user)
        .get_result::<User>(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

// TODO: require cookie auth
#[put("/user/<user_id>", format = "application/json", data = "<user>")]
pub fn modify_user(
    user_id: i64,
    user: Json<User>,
    db: db::DbConn,
) -> Result<&'static str, io::Error> {
    Ok("")
}
/// Removes a user
// TODO: require cookie auth
#[delete("/user/<user_id>")]
pub fn remove_user(user_id: i64, db: db::DbConn) -> Result<&'static str, io::Error> {
    use schema::users::dsl;
    // Remove user from database
    diesel::delete(dsl::users.filter(dsl::user_id.eq(user_id)))
        .execute(&*db)
        .map(|_| "")
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

/// Manages users
pub struct UserManager {
    cookies: HashMap<i64, String>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            cookies: HashMap::new(),
        }
    }
    pub fn cookie_is_valid(&self, user_id: i64, cookie: String) -> bool {
        self.cookies.get(&user_id) == Some(&cookie)
    }
    pub fn generate_cookie(&mut self, user_id: i64) -> String {
        // Generate random bytes
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
        // Encode bytes as hex
        let cookie = hex::encode(&bytes);
        // Store the cookie
        self.cookies.insert(user_id, cookie.clone());
        // Return the cookie
        cookie
    }
}

/// Request to log in
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    /// username
    username: String,
    /// password
    password: String,
}
/// Response to a login request
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    /// Whether the login succeeded
    success: bool,
}
/// Attempts to log a user in
#[post("/login", format = "application/json", data = "<login_data>")]
pub fn login(
    login_data: Json<LoginRequest>,
    db: db::DbConn,
) -> Result<Json<LoginResponse>, io::Error> {
    use schema::users::dsl;
    // Load entry for user
    dsl::users
        .filter(dsl::name.eq(login_data.username.clone()))
        .limit(1)
        .load::<User>(&*db)
        // Convert the error to io::Error
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        // Only take the first task
        .and_then(|task| {
            task.into_iter()
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to find matching task"))
        })
        // Verify the user's password
        .and_then(|user| {
            bcrypt::verify(&(login_data.password), &user.password)
                // Return both success and user id
                .map(|success| (success, user.user_id))
                // Fix error
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        // Create response
        .map(|(success, user_id)| Json(LoginResponse { success }))
}
