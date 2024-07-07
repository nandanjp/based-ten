mod models;
use csv::Error;
use models::media::Anime;
use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use time::macros::format_description;
use time::Date;

r#"INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon) VALUES($1, $2, $3, $4, $5) RETURNING id, title, mediaimage, numepisodes, createdon"#
"INSERT INTO VideoGames(id, title, mediaimage, createdon, console) VALUES($1, $2, $3, $4, $5) RETURNING id, title, mediaimage, createdon, console"
"INSERT INTO MOVIES(id, title, mediaimage, createdon) VALUES($1, $2, $3, $4) RETURNING id, title, mediaimage, createdon"
"INSERT INTO Songs(id, title, author, mediaimage, createdon) VALUES($1, $2, $3, $4, $5) RETURNING id, title, author, mediaimage, createdon"
"INSERT INTO Follows(followeremail, followingemail, user"

fn convert_date(created_on: String) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(created_on.as_str(), &format).map_err(|e| {
        format!("failed to create anime due to an invalid date string provided: {e:#?}")
    })
}

fn read_csv(name: String) -> Result<String, String> {
    fs::read_to_string(format !("./data/{}", name))
        .map_err(|e| format!("failed to read csv file due to the following error: {e:#?}"))
}

async fn insert_into_anime(pool: &sqlx::PgPool) {
    let csv = read_csv(String::from_str("Anime")).expect("failed to read in the csv contents and thus to insert data into anime");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Anime = record.expect("failed to deserialize the anime csv row into Anime type");
        sqlx::query!("INSERT INTO Anime(id, title")
    }
}

fn insert_into_table(table_name: String) {
    let csv = read_csv(table_name.clone()).expect("failed to read in the csv contents and thus to insert data into the given table");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Anime = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description
        );
    }
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
