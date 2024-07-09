mod models;
use clap::Parser;
use csv::ReaderBuilder;
use models::list_type::ListType;
use models::lists::{List, ListItem};
use models::media::{Anime, Games, Movie, Song};
use models::users::{Follows, Group, GroupMember, Likes, User};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::fs::File;
use std::time::Duration;
use time::macros::format_description;
use time::Date;

fn convert_date(created_on: String) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(created_on.as_str(), &format).map_err(|e| {
        format!("failed to create anime due to an invalid date string provided: {e:#?}")
    })
}

enum InsertType {
    Anime,
    Games,
    Movies,
    Songs,
    Users,
    Groups,
    GroupMembers,
    Likes,
    Lists,
    ListItems,
    Follows,
}

fn read_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut anime = Vec::new();
    for result in reader.deserialize() {
        let a: T = result?;
        anime.push(a);
    }
    Ok(anime)
}

#[derive(Clone, Copy)]
enum DataType {
    Dev,
    Prod,
}

async fn insert_into(
    pool: &sqlx::PgPool,
    data: DataType,
    table: InsertType,
) -> Result<(), Box<dyn Error>> {
    match table {
        InsertType::Anime => {
            let anime = read_csv::<Anime>(match data {
                DataType::Dev => "./data/dev/anime.csv",
                DataType::Prod => "./data/prod/anime.csv",
            })?;
            for a in anime {
                match sqlx::query!(r#"INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon) VALUES($1, $2, $3, $4, $5)"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).execute(pool).await {
                    Err(e) => println!("failed to insert into anime due to the following error = {e:#?}"),
                    _ => {}
                };
            }
        }
        InsertType::Games => {
            let games = read_csv::<Games>(match data {
                DataType::Dev => "./data/dev/video-games.csv",
                DataType::Prod => "./data/prod/video-games.csv",
            })?;
            for g in games {
                match sqlx::query!(r#"INSERT INTO VideoGames(id, title, mediaimage, console, createdon) VALUES($1, $2, $3, $4, $5)"#, g.game_id, g.title, g.image, g.platform, convert_date(g.released_on)?).execute(pool).await {
                    Err(e) => println!("failed to insert into games due to the following error = {e:#?}"),
                    _ => {}
                }
            }
        }
        InsertType::Movies => {
            let movies = read_csv::<Movie>(match data {
                DataType::Dev => "./data/dev/movies.csv",
                DataType::Prod => "./data/prod/movies.csv",
            })?;
            for m in movies {
                match sqlx::query!(
                    r#"INSERT INTO Movies(id, title, mediaimage, createdOn) VALUES($1, $2, $3, $4)"#,
                    m.movie_id,
                    m.title,
                    m.image,
                    convert_date(m.released_on)?,
                )
                .execute(pool)
                .await {
                    Err(e) => println!("failed to insert into movies due to the following error = {e:#?}"),
                    _ => {}
                };
            }
        }
        InsertType::Songs => {
            let songs = read_csv::<Song>(match data {
                DataType::Dev => "./data/dev/songs.csv",
                DataType::Prod => "./data/prod/songs.csv",
            })?;
            for s in songs {
                match sqlx::query!(r#"INSERT INTO Songs(id, title, author, album, mediaimage, createdon) VALUES($1, $2, $3, $4, $5, $6)"#, s.song_id, s.title, s.writer, s.album, s.image, convert_date(s.released_in_year)?).execute(pool).await {
                    Err(e) => println!("failed to insert into songs due to the following error = {e:#?}"),
                    _ => {}
                }
            }
        }
        InsertType::Lists => {
            let lists = read_csv::<List>(match data {
                DataType::Dev => "./data/dev/lists.csv",
                DataType::Prod => "./data/prod/lists.csv",
            })?;
            for l in lists {
                match sqlx::query!(
                    r#"INSERT INTO Lists(userName, listName, listType) VALUES($1, $2, $3)"#,
                    l.user_name,
                    l.list_name,
                    l.list_type as ListType
                )
                .execute(pool)
                .await
                {
                    Err(e) => {
                        println!("failed to insert into lists due to the following error = {e:#?}")
                    }
                    _ => {}
                };
            }
        }
        InsertType::Users => {
            let users = read_csv::<User>(match data {
                DataType::Dev => "./data/dev/users.csv",
                DataType::Prod => "./data/prod/users.csv",
            })?;
            for u in users {
                match sqlx::query!(
                    r#"INSERT INTO Users(email, username, userpassword) VALUES($1, $2, $3)"#,
                    u.user_email,
                    u.user_name,
                    u.password
                )
                .execute(pool)
                .await
                {
                    Err(e) => {
                        println!("failed to insert into users due to the following error = {e:#?}")
                    }
                    _ => {}
                };
            }
        }
        InsertType::Groups => {
            let groups = read_csv::<Group>(match data {
                DataType::Dev => "./data/dev/groups.csv",
                DataType::Prod => "./data/prod/groups.csv",
            })?;
            for g in groups {
                match sqlx::query!(
                    r#"INSERT INTO Groups(gid, groupName, ownedBy) VALUES($1, $2, $3)"#,
                    g.group_id,
                    g.name,
                    g.user_name
                )
                .execute(pool)
                .await
                {
                    Err(e) => {
                        println!("failed to insert into groups due to the following error = {e:#?}")
                    }
                    _ => {}
                };
            }
        }
        InsertType::GroupMembers => {
            let group_members = read_csv::<GroupMember>(match data {
                DataType::Dev => "./data/dev/group-members.csv",
                DataType::Prod => "./data/prod/group-members.csv",
            })?;
            for gm in group_members {
                match sqlx::query!(
                    r#"INSERT INTO GroupMembers(gid, userName) VALUES($1, $2)"#,
                    gm.group_id,
                    gm.member_name
                )
                .execute(pool)
                .await
                {
                    Err(e) => println!(
                        "failed to insert into group members due to the following error = {e:#?}"
                    ),
                    _ => {}
                };
            }
        }
        InsertType::Likes => {
            let likes = read_csv::<Likes>(match data {
                DataType::Dev => "./data/dev/likes.csv",
                DataType::Prod => "./data/prod/likes.csv",
            })?;
            for l in likes {
                match sqlx::query!(
                    r#"INSERT INTO Likes(likerName, likingName, listname) VALUES($1, $2, $3)"#,
                    l.liker_name,
                    l.liking_name,
                    l.list_name
                )
                .execute(pool)
                .await
                {
                    Err(e) => {
                        println!("failed to insert into likes due to the following error = {e:#?}")
                    }
                    _ => {}
                };
            }
        }
        InsertType::ListItems => {
            let list_items = read_csv::<ListItem>(match data {
                DataType::Dev => "./data/dev/list-items.csv",
                DataType::Prod => "./data/prod/list-items.csv",
            })?;
            for li in list_items {
                match sqlx::query!(r#"INSERT INTO ListItems(userName, listname, rankinginlist, itemid) VALUES($1, $2, $3, $4)"#, li.user_name, li.list_name, li.rank_in_list, li.item_id).execute(pool).await {
                    Err(e) => println!("failed to insert into list items due to the following error = {e:#?}"),
                    _ => {}
                };
            }
        }
        InsertType::Follows => {
            let follows = read_csv::<Follows>(match data {
                DataType::Dev => "./data/dev/follows.csv",
                DataType::Prod => "./data/prod/follows.csv",
            })?;
            for f in follows {
                match sqlx::query!(
                    r#"INSERT INTO Follows(follower, following) VALUES($1, $2)"#,
                    f.follower,
                    f.following
                )
                .execute(pool)
                .await
                {
                    Err(e) => println!(
                        "failed to insert into follows due to the following error = {e:#?}"
                    ),
                    _ => {}
                };
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which dataset to load into the database
    #[arg(short, long, default_value_t = String::from("dev"))]
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("expected environment variables/failed to parse the .env file");
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

    insert_into(&pool, data, InsertType::Anime).await?;
    println!("Finished inserting into Anime Table");
    insert_into(&pool, data, InsertType::Games).await?;
    println!("Finished inserting into Games Table");
    insert_into(&pool, data, InsertType::Movies).await?;
    println!("Finished inserting into Movies Table");
    insert_into(&pool, data, InsertType::Songs).await?;
    println!("Finished inserting into Songs Table");
    insert_into(&pool, data, InsertType::Users).await?;
    println!("Finished inserting into Users Table");
    insert_into(&pool, data, InsertType::Groups).await?;
    println!("Finished inserting into Groups Table");
    insert_into(&pool, data, InsertType::Lists).await?;
    println!("Finished inserting into Lists Table");
    insert_into(&pool, data, InsertType::GroupMembers).await?;
    println!("Finished inserting into GroupMembers Table");
    insert_into(&pool, data, InsertType::ListItems).await?;
    println!("Finished inserting into ListItems Table");
    insert_into(&pool, data, InsertType::Follows).await?;
    println!("Finished inserting into Follows Table");
    insert_into(&pool, data, InsertType::Likes).await?;
    println!("Finished inserting into Likes Table");
    Ok(())
}
