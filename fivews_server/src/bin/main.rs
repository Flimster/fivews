use fivewsdb::db::FiveWsDB;
use fivews_server::paths::create_paths;

#[tokio::main]
async fn main() {
    let db = FiveWsDB::new("./lidb");
    warp::serve(create_paths(db))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
