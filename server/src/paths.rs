use fivewsdb::db::FiveWsDB;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{body::json, hyper::StatusCode, reply::Json, Filter};

use crate::models::*;

type ServerDB = Arc<RwLock<FiveWsDB>>;

fn with_db(db: ServerDB) -> impl Filter<Extract = (Arc<RwLock<FiveWsDB>>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn list_entries(query: DbQuery, db: ServerDB) -> Result<Json, Infallible> {
    let entries: Vec<LogEntry> = db
        .read()
        .await
        .read(query.query.as_str())
        .into_iter()
        .map(|entry| LogEntry {
            who: entry.who,
            what: entry.what,
            when: entry.when,
            r#where: entry.r#where,
            why: entry.why,
        })
        .collect();
    Ok(warp::reply::json(&entries))
}

fn read(db: ServerDB) -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Clone {
    warp::path!("read")
        .and(warp::get())
        .and(warp::query::<DbQuery>())
        .and(with_db(db))
        .and_then(list_entries)
}

async fn update_database(log_entry: LogEntry, db: ServerDB) -> Result<StatusCode, Infallible> {
    db.write()
        .await
        .update(
            log_entry.who,
            log_entry.what,
            log_entry.when,
            log_entry.r#where,
            log_entry.why,
        )
        .unwrap();

    Ok(StatusCode::CREATED)
}

fn update(db: Arc<RwLock<FiveWsDB>>) -> impl Filter<Extract = (StatusCode,), Error = warp::Rejection> + Clone {
    warp::path!("update")
        .and(warp::post())
        .and(json())
        .and(with_db(db))
        .and_then(update_database)
}

pub fn create_paths(db: FiveWsDB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Initializing a database with two phase locking
    // This allows multiple readers but only 1 writer
    let concurrent_db = Arc::new(RwLock::new(db));
    update(concurrent_db.clone()).or(read(concurrent_db))
}
