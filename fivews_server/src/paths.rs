use fivewsdb::db::FiveWsDB;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{body::json, hyper::StatusCode, Filter};

use crate::models::*;

fn with_db(
    db: Arc<RwLock<FiveWsDB>>,
) -> impl Filter<Extract = (Arc<RwLock<FiveWsDB>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn list_entries(
    query: DbQuery,
    db: Arc<RwLock<FiveWsDB>>,
) -> Result<impl warp::Reply, Infallible> {
    let entries: Vec<String> = db
        .read()
        .await
        .read(query.query.as_str())
        .iter()
        .map(|entry| entry.to_string())
        .collect();
    Ok(warp::reply::json(&entries))
}

fn read(
    db: Arc<RwLock<FiveWsDB>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("read")
        .and(warp::get())
        .and(warp::query::<DbQuery>())
        .and(with_db(db))
        .and_then(list_entries)
}

async fn update_database(
    log_entry: LogEntry,
    db: Arc<RwLock<FiveWsDB>>,
) -> Result<impl warp::Reply, Infallible> {
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

fn update(
    db: Arc<RwLock<FiveWsDB>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("update")
        .and(warp::post())
        .and(json())
        .and(with_db(db))
        .and_then(update_database)
}

pub fn create_paths(db: FiveWsDB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Init the database
    let concurrent_db = Arc::new(RwLock::new(db));
    update(concurrent_db.clone()).or(read(concurrent_db))
}
