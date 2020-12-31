use fivewsdb::db::FiveWsDB;
use fivews_server::models::*;
use fivews_server::paths::create_paths;

use warp::http::StatusCode;
use warp::test::request;

pub fn teardown(path: &str) {
    println!("Cleaning files. Path: '{}'", path);
    // We want the tests to quit if we fail to tear everything down
    // Otherwise the rest of the integration tests might fail
    std::fs::remove_dir_all(path).expect("Failed to teardown directory");
}

#[tokio::test]
async fn test_server_update() {
    let db = FiveWsDB::new("./test_update_db");
    let paths = create_paths(db);

    let resp = request()
        .method("POST")
        .path("/update")
        .json(&LogEntry {
            who: String::from("w"),
            what: String::from("w"),
            when: String::from("w"),
            r#where: String::from("w"),
            why: String::from("w"),
        })
        .reply(&paths)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    teardown("./test_update_db");
}

#[tokio::test]
async fn test_server_read() {

    let mut db = FiveWsDB::new("./test_read_db");

    db.update("w", "w", "w", "w", "w").unwrap();
    db.update("w", "w", "w", "w", "w").unwrap();

    let paths = create_paths(db);
    let res  = request()
        .method("GET")
        .path("/read?query=*")
        .reply(&paths)
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    teardown("./test_read_db");
    
}
