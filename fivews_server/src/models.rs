use serde::{Deserialize, Serialize};

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
