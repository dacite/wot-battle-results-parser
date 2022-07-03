use std::env;
mod db;
mod schema;
use schema::Replay;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let replay_db = db::ReplayDatabase::new(pool);


    let replay = Replay::new("replay_parser/input_files/20200623_0522_usa-A92_M60_36_fishing_bay.wotreplay");
    replay_db.insert_replay(replay).await;

    // println!("{:#?}", replay);

    Ok(())
}
