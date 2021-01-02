use dirs;
use fivewsdb::db::FiveWsDB;
use server::paths::create_paths;

#[tokio::main]
async fn main() {
    let dir = format!("{}/.lidb", dirs::home_dir().unwrap().to_str().unwrap());
    let db = FiveWsDB::new(dir.as_str());
    let port = 6211;
    let ip = [127, 0, 0, 1];

    println!("Starting server on port: {}", port);
    warp::serve(create_paths(db)).run((ip, port)).await;
}
