use schema::{tasks, users, work};

/// A task that can be worked on
#[derive(Deserialize, Queryable, Serialize)]
pub struct Task {
    /// UID of the task
    task_id: i64,
    /// ID of the user who created the event
    owner_id: i64,
    /// Name of the event
    name: String,
    /// Description of the event
    description: String,
    /// When the event was created
    /// TODO: change to timestamp type
    created: i64,
    /// When the event is due
    /// TODO: change to timestamp type
    due: i64,
    /// How important it is that this event is completed
    importance: f64,
    /// Whether the event is currently being worked on
    in_progress: bool,
    /// Current event progress
    progress: f64,
    /// Whether the event has been completed
    completed: bool,
}

/// Representation of a task that is inserted into the database
#[derive(Deserialize, Queryable, Insertable, Serialize)]
#[table_name = "tasks"]
pub struct TaskInsert {
    /// ID of the user who created the event
    owner_id: i64,
    /// Name of the event
    name: String,
    /// Description of the event
    description: String,
    /// When the event was created
    /// TODO: change to timestamp type
    created: i64,
    /// When the event is due
    /// TODO: change to timestamp type
    due: i64,
    /// How important it is that this event is completed
    importance: f64,
}

/// A user of the system
#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    /// Unique identifier
    pub user_id: i64,
    /// username
    pub name: String,
    /// password
    /// TODO: hash and salt
    pub password: String,
}

/// Work done on a task
#[derive(Deserialize, Queryable, Serialize)]
pub struct Work {
    /// uid of this work item
    work_id: i64,
    /// uid of the work this is being done o
    task_id: i64,
    /// Time the user started this work
    start_time: i64,
    /// Amount of time the user worked
    end_time: i64,
    /// Amount of progress the user made
    progress: f64,
    /// Whether the user finished the task with this work session
    finished: bool,
    /// Whether the user listened to music during this work session
    music: Option<bool>,
    /// Number of times the user was interrupted during this work session
    interruptions: Option<i64>,
    /// How noisy was the area during this work session
    noise: Option<f64>,
    /// How many meetings occurred during the work session
    meetings: Option<i64>,
    /// Number of breaks taken
    breaks: Option<i64>,
}
/// Data inserted for starting work
#[derive(Deserialize, Insertable, Serialize)]
#[table_name = "work"]
pub struct WorkStart {
    /// uid of the work this is being done o
    pub task_id: i64,
    /// Time the user started this work
    pub start_time: i64,
    /// Time the user ended this work. This will just be -1, since the user hasn't ended work yet
    pub end_time: i64,
}
/// Data used to finish work
#[derive(Deserialize, Serialize)]
pub struct WorkFinish {
    /// End time
    pub end_time: Option<i64>,
    /// Amount of progress the user made
    pub progress: f64,
    /// Whether the user finished the task with this work session
    pub finished: bool,
    /// Whether the user listened to music during this work session
    pub music: Option<bool>,
    /// Number of times the user was interrupted during this work session
    pub interruptions: Option<i64>,
    /// How noisy was the area during this work session
    pub noise: Option<f64>,
    /// How many meetings occurred during the work session
    pub meetings: Option<i64>,
    /// Number of breaks taken
    pub breaks: Option<i64>,
}

/// Version of work struct for inserting into table
#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "work"]
pub struct WorkInsert {
    /// uid of the work this is being done o
    task_id: i64,
    /// Time the user started this work
    start_time: i64,
    /// Amount of time the user worked
    end_time: i64,
    /// Amount of progress the user made
    progress: f64,
    /// Whether the user finished the task with this work session
    finished: bool,
    /// Whether the user listened to music during this work session
    music: Option<bool>,
    /// Number of times the user was interrupted during this work session
    interruptions: Option<i64>,
    /// How noisy was the area during this work session
    noise: Option<f64>,
    /// How many meetings occurred during the work session
    meetings: Option<i64>,
    /// Number of breaks taken
    breaks: Option<i64>,
}
