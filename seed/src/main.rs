mod models;
use csv::ReaderBuilder;
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

async fn insert_into(pool: &sqlx::PgPool, table: InsertType) -> Result<(), Box<dyn Error>> {
    match table {
        InsertType::Anime => {
            let anime = read_csv::<Anime>("Anime")?;
            for a in anime {
                let _ = sqlx::query!(r#"INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon) VALUES($1, $2, $3, $4, $5)"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
        InsertType::Games => {
            let games = read_csv::<Games>("VideoGames")?;
            for g in games {
                let _ = sqlx::query!(r#"INSERT INTO VideoGames(id, title, mediaimage, console, createdon) VALUES($1, $2, $3, $4, $5)"#, g.game_id, g.title, g.image, convert_date(g.released_on)?, g.platform).fetch_one(pool).await?;
            }
        }
        InsertType::Movies => {
            let movies = read_csv::<Movie>("Movies")?;
            for m in movies {
                let _ = sqlx::query!(
                    r#"INSERT INTO Movies(id, title, mediaimage, createdOn) VALUES($1, $2, $3, $4)"#,
                    m.movie_id,
                    m.title,
                    m.image,
                    convert_date(m.released_on)?,
                )
                .fetch_one(pool)
                .await?;
            }
        }
        InsertType::Songs => {
            let songs = read_csv::<Song>("Songs")?;
            for s in songs {
                let _ = sqlx::query!(r#"INSERT INTO Songs(id, title, author, album, mediaimage, createdOn) VALUES($1, $2, $3, $4, $5, $6)"#, s.song_id, s.title, s.writer, a.album, a.image, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
        InsertType::Lists => {
            let lists = read_csv::<List>("Lists")?;
            for l in lists {
                let _ = sqlx::query!(
                    r#"INSERT INTO Lists(email, listName, listType) VALUES($1, $2, $3)"#,
                    l.user_email,
                    l.list_name,
                    l.list_type
                )
                .fetch_one(pool)
                .await?;
            }
        }
        InsertType::Users => {
            let users = read_csv::<User>("Users")?;
            for u in users {
                let _ = sqlx::query!(
                    r#"INSERT INTO Users(email, displayname, userpassword) VALUES($1, $2, $3)"#,
                    u.user_email,
                    u.user_name,
                    u.password
                )
                .fetch_one(pool)
                .await?;
            }
        }
        InsertType::Groups => {
            let groups = read_csv::<Group>("Groups")?;
            for g in groups {
                let _ = sqlx::query!(r#"INSERT INTO Groups(groupName,ownedBy) VALUES($1, $2) RETURNING gid, groupName, ownedBy"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
        InsertType::GroupMembers => {
            let group_members = read_csv::<GroupMember>("GroupMembers")?;
            for gm in group_members {
                let _ = sqlx::query!(
                    r#"INSERT INTO GroupMembers(gid, email) VALUES($1, $2) RETURNING gid, email"#,
                    a.anime_id,
                    a.title,
                    a.image,
                    a.episodes,
                    convert_date(a.released_on)?
                )
                .fetch_one(pool)
                .await?;
            }
        }
        InsertType::Likes => {
            let likes = read_csv::<Likes>("Likes")?;
            for l in likes {
                let _ = sqlx::query!(r#"INSERT INTO Likes(likerEmail, likingEmail, listName) VALUES($1, $2, $3) RETURNING likerEmail, likingEmail, listName"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
        InsertType::ListItems => {
            let list_items = read_csv::<ListItem>("ListItems")?;
            for li in list_items {
                let _ = sqlx::query!(r#"INSERT INTO ListItems(email, listName, itemID, rankingInList) VALUES($1, $2, $3, $4) RETURNING email, listName, itemID, rankingInList"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
        InsertType::Follows => {
            let follows = read_csv::<Follows>("Follows")?;
            for f in follows {
                let _ = sqlx::query!(r#"INSERT INTO Follows(followerEmail, followingEmail) VALUES($1, $2) RETURNING followerEmail, followingEmail"#, a.anime_id, a.title, a.image, a.episodes, convert_date(a.released_on)?).fetch_one(pool).await?;
            }
        }
    }
    Ok(())
}

async fn insert_into_anime(pool: &sqlx::PgPool, table: InsertType) {
    let csv = read_csv(match table {
        InsertType::Anime => "Anime",
        InsertType::Games => "VideGames",
        InsertType::Movies => "Movies",
        InsertType::Songs => "Songs",
        InsertType::Lists => "Lists",
        InsertType::Users => "Users",
        InsertType::Groups => "Groups",
        InsertType::GroupMembers => "GroupMembers",
        InsertType::Likes => "Lists",
        InsertType::ListItems => "ListItems",
        InsertType::Follows => "Follows",
    })
    .expect("failed to read in the csv contents and thus to insert data into anime");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
