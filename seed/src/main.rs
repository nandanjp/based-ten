mod models;
mod utils;
use clap::Parser;
use models::lists::{List, ListItem};
use models::media::{Anime, Games, Movie, Song};
use models::traits::Commit;
use models::users::{Follows, Group, GroupMember, Likes, User};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;
use utils::{read_csv, DataType, InsertType};

async fn insert_into<T>(pool: &sqlx::PgPool, data: &str) -> Result<(), Box<dyn Error>>
where
    T: Commit + for<'de> Deserialize<'de>,
{
    let values = read_csv::<T::Value>(data)?;
    T::commit(&pool, values).await
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which dataset to load into the database
    #[arg(short, long, default_value_t = String::from("dev"))]
    data: String,
}

fn populate_table_to_data(
    p: &Path,
    table: &mut Vec<(InsertType, String, String)>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(p)? {
        let path = entry?.path();
        let insert_type = InsertType::from_str(
            &path
                .file_stem()
                .map(|s| String::from(s.to_str().unwrap()))
                .unwrap()
                .split('.')
                .nth(1)
                .unwrap_or(""),
        );
        table.push((
            insert_type?,
            path.to_str().map(String::from).unwrap(),
            path.file_stem()
                .map(|s| String::from(s.to_str().unwrap()))
                .unwrap(),
        ));
    }

    table.sort_by(|(_, __, a), (___, ____, b)| {
        a.split('.')
            .nth(0)
            .unwrap()
            .parse::<u8>()
            .expect("did not get a number")
            .cmp(
                &b.split('.')
                    .nth(0)
                    .unwrap()
                    .parse::<u8>()
                    .expect("did not get a number"),
            )
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("expected environment variables/failed to parse the .env file");

    let mut table_to_dev_data: Vec<(InsertType, String, String)> = Vec::new();
    let path_to_dev_data = Path::new("./data/dev");
    let mut table_to_prod_data: Vec<(InsertType, String, String)> = Vec::new();
    let path_to_prod_data = Path::new("./data/prod");

    let data = match Args::parse().data.as_str() {
        "prod" => DataType::Prod,
        _ => DataType::Dev,
    };

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

    //ORDER OF INSERT: anime, movies, songs, users, video-games, groups, lists, group-members, list-itesm, follows, likes
    match data {
        DataType::Dev => populate_table_to_data(&path_to_dev_data, &mut table_to_dev_data)?,
        DataType::Prod => populate_table_to_data(&path_to_prod_data, &mut table_to_prod_data)?,
    };
    for (table, data, _) in match data {
        DataType::Dev => &table_to_dev_data,
        DataType::Prod => &table_to_prod_data,
    } {
        let table_name = table.to_str();
        match table {
            InsertType::Anime => insert_into::<Anime>(&pool, &data).await?,
            InsertType::Follows => insert_into::<Follows>(&pool, &data).await?,
            InsertType::Games => insert_into::<Games>(&pool, &data).await?,
            InsertType::GroupMembers => insert_into::<GroupMember>(&pool, &data).await?,
            InsertType::Groups => insert_into::<Group>(&pool, &data).await?,
            InsertType::Likes => insert_into::<Likes>(&pool, &data).await?,
            InsertType::ListItems => insert_into::<ListItem>(&pool, &data).await?,
            InsertType::Lists => insert_into::<List>(&pool, &data).await?,
            InsertType::Movies => insert_into::<Movie>(&pool, &data).await?,
            InsertType::Songs => insert_into::<Song>(&pool, &data).await?,
            InsertType::Users => insert_into::<User>(&pool, &data).await?,
        }

        println!("Successfully Inserted into the {} Table", table_name);
    }

    Ok(())
}
