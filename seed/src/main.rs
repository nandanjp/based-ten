mod models;
use csv::Error;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use time::macros::format_description;
use time::Date;

pub fn convert_date(created_on: String) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(created_on.as_str(), &format).map_err(|e| {
        format!("failed to create anime due to an invalid date string provided: {e:#?}")
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().expect("expected environment variables/failed to parse the .env file");

    let db_str = std::env::var("DATABASE_URL")
        .expect("Failed to find DATABASE_URL")
        .to_string();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_str)
        .await
        .expect(&format!(
            "failed to connect to the database using the provided connection string: {db_str}"
        ));

    let anime = sqlx::query!("SELECT * FROM Anime")
        .fetch_all(&pool)
        .await
        .map(|a| {
            a.into_iter()
                .map(|a| {
                    format!(
                        "name = {}, number of episodes = {}, created on = {}",
                        a.title,
                        a.numepisodes.unwrap(),
                        a.createdon.unwrap().to_string()
                    )
                })
                .collect::<Vec<String>>()
        })
        .expect("failed to retrieve anime from the database :((((");

    println!("anime from database: = {:?}", anime);
    Ok(())
}
