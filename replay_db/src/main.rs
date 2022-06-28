use std::env;
mod schema;
use schema::Replay;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;


    let replay = Replay::new("replay_parser/input_files/example.wotreplay");

    println!("{:#?}", replay);

    Ok(())
}
