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

#[derive(Clone, Copy, Debug)]
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
            let anime_ids = anime
                .iter()
                .map(|a| a.anime_id as i64)
                .collect::<Vec<i64>>();
            let titles = anime
                .iter()
                .map(|a| a.title.clone())
                .collect::<Vec<String>>();
            let images = anime
                .iter()
                .map(|a| a.image.clone())
                .collect::<Vec<String>>();
            let episodes = anime
                .iter()
                .map(|a| a.episodes as i64)
                .collect::<Vec<i64>>();
            let dates = anime
                .iter()
                .map(|a| match convert_date(a.released_on.clone()) {
                    Ok(date) => date,
                    Err(e) => {
                        panic!("failed to parse the date due to the following error: {e:#?}, anime = {}", a.anime_id)
                    }
                })
                .collect::<Vec<Date>>();
            let _ = sqlx::query!(
                r#"
                    INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon)
                    SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::int8[], $5::Date[])
                "#,
                &anime_ids[..],
                &titles[..],
                &images[..],
                &episodes[..],
                &dates[..] as &[Date]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Games => {
            let games = read_csv::<Games>(match data {
                DataType::Dev => "./data/dev/video-games.csv",
                DataType::Prod => "./data/prod/video-games.csv",
            })?;
            let game_ids = games.iter().map(|g| g.game_id as i64).collect::<Vec<i64>>();
            let titles = games
                .iter()
                .map(|g| g.title.clone())
                .collect::<Vec<String>>();
            let images = games
                .iter()
                .map(|g| g.image.clone())
                .collect::<Vec<String>>();
            let platforms = games
                .iter()
                .map(|g| g.platform.clone())
                .collect::<Vec<String>>();
            let dates = games
                .iter()
                .map(|g| convert_date(g.released_on.clone()).expect("failed to parse date"))
                .collect::<Vec<Date>>();

            let _ = sqlx::query!(
                r#"
                INSERT INTO VideoGames(id, title, mediaimage, console, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::text[], $5::Date[])
            "#,
                &game_ids[..],
                &titles[..],
                &images[..],
                &platforms[..],
                &dates[..] as &[Date]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Movies => {
            let movies = read_csv::<Movie>(match data {
                DataType::Dev => "./data/dev/movies.csv",
                DataType::Prod => "./data/prod/movies.csv",
            })?;

            let movie_ids = movies
                .iter()
                .map(|m| m.movie_id as i64)
                .collect::<Vec<i64>>();
            let titles = movies
                .iter()
                .map(|m| m.title.clone())
                .collect::<Vec<String>>();
            let images = movies
                .iter()
                .map(|m| m.image.clone())
                .collect::<Vec<String>>();
            let dates = movies
                .iter()
                .map(|m| {
                    convert_date(m.released_on.clone())
                        .expect("failed to convert the string to a date")
                })
                .collect::<Vec<Date>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Movies(id, title, mediaimage, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::Date[])
            "#,
                &movie_ids[..],
                &titles[..],
                &images[..],
                &dates[..] as &[Date]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Songs => {
            let songs = read_csv::<Song>(match data {
                DataType::Dev => "./data/dev/songs.csv",
                DataType::Prod => "./data/prod/songs.csv",
            })?;
            let song_ids = songs.iter().map(|s| s.song_id as i64).collect::<Vec<i64>>();
            let titles = songs
                .iter()
                .map(|s| s.title.clone())
                .collect::<Vec<String>>();
            let writers = songs
                .iter()
                .map(|s| s.writer.clone())
                .collect::<Vec<String>>();
            let albums = songs
                .iter()
                .map(|s| s.album.clone())
                .collect::<Vec<String>>();
            let images = songs
                .iter()
                .map(|s| s.image.clone())
                .collect::<Vec<String>>();
            let dates = songs
                .iter()
                .map(|s| convert_date(s.released_in_year.clone()).expect("failed to parse date"))
                .collect::<Vec<Date>>();
            let _ = sqlx::query!(r#"
                INSERT INTO Songs(id, title, author, album, mediaimage, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::text[], $5::text[], $6::Date[])
            "#, &song_ids[..], &titles[..], &writers[..], &albums[..], &images[..], &dates[..] as &[Date]).execute(pool).await?;
        }
        InsertType::Lists => {
            let lists = read_csv::<List>(match data {
                DataType::Dev => "./data/dev/lists.csv",
                DataType::Prod => "./data/prod/lists.csv",
            })?;

            let user_names = lists
                .iter()
                .map(|l| l.user_name.clone())
                .collect::<Vec<String>>();
            let list_names = lists
                .iter()
                .map(|l| l.list_name.clone())
                .collect::<Vec<String>>();
            let list_types = lists
                .iter()
                .map(|l| l.list_type.clone())
                .collect::<Vec<ListType>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Lists(username, listname, listtype)
                SELECT * FROM UNNEST($1::text[], $2::text[], $3::ListType[])
            "#,
                &user_names[..],
                &list_names[..],
                &list_types[..] as &[ListType]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Users => {
            let users = read_csv::<User>(match data {
                DataType::Dev => "./data/dev/users.csv",
                DataType::Prod => "./data/prod/users.csv",
            })?;
            let user_emails = users
                .iter()
                .map(|u| u.user_email.clone())
                .collect::<Vec<String>>();
            let user_names = users
                .iter()
                .map(|u| u.user_name.clone())
                .collect::<Vec<String>>();
            let passwords = users
                .iter()
                .map(|u| u.password.clone())
                .collect::<Vec<String>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Users(email, username, userpassword)
                SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])
            "#,
                &user_emails[..],
                &user_names[..],
                &passwords[..]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Groups => {
            let groups = read_csv::<Group>(match data {
                DataType::Dev => "./data/dev/groups.csv",
                DataType::Prod => "./data/prod/groups.csv",
            })?;
            let group_ids = groups
                .iter()
                .map(|g| g.group_id as i64)
                .collect::<Vec<i64>>();
            let group_names = groups
                .iter()
                .map(|g| g.name.clone())
                .collect::<Vec<String>>();
            let owners = groups
                .iter()
                .map(|g| g.user_name.clone())
                .collect::<Vec<String>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Groups(gid, groupname, ownedby)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[])
            "#,
                &group_ids[..],
                &group_names[..],
                &owners[..]
            )
            .execute(pool)
            .await?;
        }
        InsertType::GroupMembers => {
            let group_members = read_csv::<GroupMember>(match data {
                DataType::Dev => "./data/dev/group-members.csv",
                DataType::Prod => "./data/prod/group-members.csv",
            })?;
            let group_ids = group_members
                .iter()
                .map(|g| g.group_id as i64)
                .collect::<Vec<i64>>();
            let user_names = group_members
                .iter()
                .map(|g| g.member_name.clone())
                .collect::<Vec<String>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO GroupMembers(gid, username)
                SELECT * FROM UNNEST($1::int8[], $2::text[])
            "#,
                &group_ids[..],
                &user_names[..]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Likes => {
            let likes = read_csv::<Likes>(match data {
                DataType::Dev => "./data/dev/likes.csv",
                DataType::Prod => "./data/prod/likes.csv",
            })?;
            let liker_names = likes
                .iter()
                .map(|l| l.liker_name.clone())
                .collect::<Vec<String>>();
            let liking_names = likes
                .iter()
                .map(|l| l.liking_name.clone())
                .collect::<Vec<String>>();
            let list_names = likes
                .iter()
                .map(|l| l.list_name.clone())
                .collect::<Vec<String>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Likes(likername, likingname, listname)
                SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])
            "#,
                &liker_names[..],
                &liking_names[..],
                &list_names[..]
            )
            .execute(pool)
            .await?;
        }
        InsertType::ListItems => {
            let list_items = read_csv::<ListItem>(match data {
                DataType::Dev => "./data/dev/list-items.csv",
                DataType::Prod => "./data/prod/list-items.csv",
            })?;

            let user_names = list_items
                .iter()
                .map(|l| l.user_name.clone())
                .collect::<Vec<String>>();
            let list_names = list_items
                .iter()
                .map(|l| l.list_name.clone())
                .collect::<Vec<String>>();
            let rank_in_lists = list_items
                .iter()
                .map(|l| l.rank_in_list as i64)
                .collect::<Vec<i64>>();
            let item_ids = list_items
                .iter()
                .map(|l| l.item_id as i64)
                .collect::<Vec<i64>>();
            let _ = sqlx::query!(
                r#"
                    INSERT INTO ListItems(username, listname, rankinginlist, itemid)
                    SELECT * FROM UNNEST($1::text[], $2::text[], $3::int8[], $4::int8[])
                "#,
                &user_names[..],
                &list_names[..],
                &rank_in_lists[..],
                &item_ids[..]
            )
            .execute(pool)
            .await?;
        }
        InsertType::Follows => {
            let follows = read_csv::<Follows>(match data {
                DataType::Dev => "./data/dev/follows.csv",
                DataType::Prod => "./data/prod/follows.csv",
            })?;

            let followers = follows
                .iter()
                .map(|f| f.follower.clone())
                .collect::<Vec<String>>();
            let followings = follows
                .iter()
                .map(|f| f.following.clone())
                .collect::<Vec<String>>();
            let _ = sqlx::query!(
                r#"
                INSERT INTO Follows(follower, following)
                SELECT * FROM UNNEST($1::text[], $2::text[])
            "#,
                &followers[..],
                &followings[..]
            )
            .execute(pool)
            .await?;
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
