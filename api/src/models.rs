use schema::{events, users};

/// A calendar event
#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "events"]
pub struct Event {
    /// UID of the event
    ///
    /// Optional in the case that the event has not yet been given an ID
    event_id: i64,
    /// ID of the user who created the event
    user_id: i64,
    /// Name of the event
    name: String,
    /// Description of the event
    description: Option<String>,
    /// Time the event begins
    /// TODO: change to timestamp type
    begin_timestamp: String,
    /// Time the event ends
    /// TODO: change to timestamp type
    end_timestamp: String,
    /// Type of recurrence
    /// TODO: enum
    recurrence: String,
    /// How important this event is
    importance: i64,
}
/// A user of the system
#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    /// Unique identifier
    user_id: i64,
    /// username
    name: String,
    /// password
    /// TODO: hash_salt
    password: String,
}
