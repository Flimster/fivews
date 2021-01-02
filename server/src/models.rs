use serde::{Deserialize, Serialize};

// TODO: Find a better solution for using the same data structures for the server and db
// Exactly like in fivewsdb/src/entry.rs, this will be fixed later
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogEntry {
    pub who: String,
    pub what: String,
    pub when: String,
    pub r#where: String,
    pub why: String,
}

#[derive(Deserialize)]
pub struct DbQuery {
    pub query: String,
}
