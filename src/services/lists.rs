use serde::Serialize;

use crate::models::lists::{
    CreateList, ErrorList, List, ListType, ListWithLikes, QueryList, UpdateList,
};

#[derive(Debug, Clone, Serialize)]
pub struct FullListItem {
    pub user_name: String,
    pub list_name: String,
    pub ranking_in_list: i32,
    pub item_id: i32,
    #[serde(rename = "type")]
    pub list_type: ListType,
    pub likes: i64,
}

pub struct ListService;
impl ListService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryList,
    ) -> Result<Vec<ListWithLikes>, ErrorList> {
        match query_obj {
            _ => sqlx::query_as!(ListWithLikes,
                r#"
                SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
                FROM Lists JOIN Likes ON Lists.listname = Likes.listname
                GROUP BY Lists.username, Lists.listname, listtype
                "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve all lists: {e:#?}"
                ))
            }),
        }
    }

    pub async fn get_by_email(
        pool: &sqlx::PgPool,
        user_name: String,
    ) -> Result<Vec<ListWithLikes>, ErrorList> {
        sqlx::query_as!(ListWithLikes, r#"
            SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
            FROM Lists JOIN Likes ON Lists.listname = Likes.listname
            WHERE username = $1
            GROUP BY Lists.username, Lists.listname, listtype
        "#, user_name)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list: {e:#?}"
                ))
            })
    }

    pub async fn get_by_user_and_listname(
        pool: &sqlx::PgPool,
        user_name: String,
        list_name: String,
    ) -> Result<ListWithLikes, ErrorList> {
        sqlx::query_as!(
            ListWithLikes,
            r#"
            SELECT username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
            FROM Lists JOIN Likes ON Lists.listname = Likes.listname
            WHERE username = $1 AND Lists.listname = $2
            GROUP BY username, Lists.listname, listtype
        "#,
            user_name,
            list_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorList(format!("failed to retrieve list: {e:#?}")))
    }

    pub async fn get_user_list_and_items(
        pool: &sqlx::PgPool,
        user_name: String,
        list_name: String,
    ) -> Result<Vec<serde_json::Value>, ErrorList> {
        let list = sqlx::query!(r#"
            SELECT listtype AS "listtype: ListType" FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname WHERE Lists.listname = $1 AND Lists.username = $2 LIMIT 1
        "#, list_name, user_name).fetch_one(pool).await.map_err(|e| ErrorList(format!("failed to retrieve all list details: {e:#?}")))?;

        match list.listtype {
            ListType::Anime => {
                sqlx::query!(r#"
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, numepisodes, createdon, COUNT(*) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname JOIN Anime ON ListItems.itemid = Anime.id
                    WHERE Lists.listname = $1 AND Lists.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, numepisodes, createdon, listtype
                "#, list_name, user_name).fetch_all(pool).await.map(|a| a.into_iter().map(|a| serde_json::json!({
                    "username": a.username,
                    "listname": a.listname,
                    "rankinginlist": a.rankinginlist,
                    "itemid": a.itemid,
                    "title": a.title,
                    "media_image": a.mediaimage,
                    "numepisodes": a.numepisodes,
                    "createdon": a.createdon,
                    "likes": a.likes,
                    "listtype": a.listtype
                })).collect::<Vec<serde_json::Value>>()).map_err(|e| ErrorList(format!("failed to retrieve all list details: {e:#?}")))
            },
            ListType::Movies => {
                sqlx::query!(r#"
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, createdon, COUNT(*) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname JOIN Movies ON ListItems.itemid = Movies.id
                    WHERE Lists.listname = $1 AND Lists.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, createdon, listtype
                "#, list_name, user_name).fetch_all(pool).await.map(|a| a.into_iter().map(|a| serde_json::json!({
                    "username": a.username,
                    "listname": a.listname,
                    "rankinginlist": a.rankinginlist,
                    "itemid": a.itemid,
                    "title": a.title,
                    "media_image": a.mediaimage,
                    "createdon": a.createdon,
                    "likes": a.likes,
                    "listtype": a.listtype
                })).collect::<Vec<serde_json::Value>>()).map_err(|e| ErrorList(format!("failed to retrieve all list details: {e:#?}")))
            },
            ListType::VideoGames => {
                sqlx::query!(r#"
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, console, createdon, COUNT(*) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname JOIN VideoGames ON ListItems.itemid = VideoGames.id
                    WHERE Lists.listname = $1 AND Lists.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, console, createdon, listtype
                "#, list_name, user_name).fetch_all(pool).await.map(|a| a.into_iter().map(|a| serde_json::json!({
                    "username": a.username,
                    "listname": a.listname,
                    "rankinginlist": a.rankinginlist,
                    "itemid": a.itemid,
                    "title": a.title,
                    "media_image": a.mediaimage,
                    "console": a.console,
                    "createdon": a.createdon,
                    "likes": a.likes,
                    "listtype": a.listtype
                })).collect::<Vec<serde_json::Value>>()).map_err(|e| ErrorList(format!("failed to retrieve all list details: {e:#?}")))
            },
            ListType::Songs => {
                sqlx::query!(r#"
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, author, album, createdon, COUNT(*) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname JOIN Songs ON ListItems.itemid = Songs.id
                    WHERE Lists.listname = $1 AND Lists.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, author, album, createdon, listtype
                "#, list_name, user_name).fetch_all(pool).await.map(|a| a.into_iter().map(|a| serde_json::json!({
                    "username": a.username,
                    "listname": a.listname,
                    "rankinginlist": a.rankinginlist,
                    "itemid": a.itemid,
                    "title": a.title,
                    "media_image": a.mediaimage,
                    "author": a.author,
                    "album": a.album,
                    "createdon": a.createdon,
                    "likes": a.likes,
                    "listtype": a.listtype
                })).collect::<Vec<serde_json::Value>>()).map_err(|e| ErrorList(format!("failed to retrieve all list details: {e:#?}")))
            }
        }
    }

    pub async fn get_explore_lists(
        pool: &sqlx::PgPool,
        user_name: String,
    ) -> Result<Vec<List>, ErrorList> {
        sqlx::query_file!("sql/feature5/explore_lists.sql", user_name)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| List {
                        username: a.username,
                        listname: a.listname,
                        listtype: a.listtype,
                    })
                    .collect::<Vec<List>>()
            })
            .map_err(|e| ErrorList(format!("failed to retrieve explore lists: {e:#?}")))
    }

    pub async fn get_top_lists(
        pool: &sqlx::PgPool,
        query_obj: QueryList,
    ) -> Result<Vec<ListWithLikes>, ErrorList> {
        match query_obj {
            QueryList {
                limit_num: Some(limit_num),
            } => sqlx::query_as!(
                ListWithLikes,
                r#"WITH ListsWithLikes AS (
                        SELECT l.*
                        FROM Lists l
                            JOIN Likes lk on l.listName = lk.listName
                    ),
                    ListLikeCounts AS (
                        SELECT lwl.listName,
                            lwl.username,
                            lwl.listType,
                            COUNT(*) as likes
                        FROM ListsWithLikes lwl
                        GROUP BY lwl.username,
                            lwl.listName,
                            lwl.listType
                    )
                    SELECT userName, listName, listtype AS "listtype: ListType", likes
                    FROM ListLikeCounts l
                    ORDER BY l.likes DESC
                    LIMIT $1"#,
                limit_num,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve all lists due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query_as!(
                ListWithLikes,
                r#"WITH ListsWithLikes AS (
                        SELECT l.*
                        FROM Lists l
                            JOIN Likes lk on l.listName = lk.listName
                    ),
                    ListLikeCounts AS (
                        SELECT lwl.listName,
                            lwl.username,
                            lwl.listType,
                            COUNT(*) as likes
                        FROM ListsWithLikes lwl
                        GROUP BY lwl.username,
                            lwl.listName,
                            lwl.listType
                    )
                    SELECT userName, listName, listtype AS "listtype: ListType", likes
                    FROM ListLikeCounts l
                    ORDER BY l.likes DESC"#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorList(format!("failed to retrieve all lists: {e:#?}"))),
        }
    }

    pub async fn create(pool: &sqlx::PgPool, create_obj: CreateList) -> Result<List, ErrorList> {
        sqlx::query_as!(
            List,
            r#"INSERT INTO Lists(userName, listName, listType) VALUES($1, $2, $3) RETURNING username, listname, listtype AS "listtype: ListType""#,
            create_obj.user_name,
            create_obj.list_name,
            create_obj.list_type as ListType
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorList(format!("failed to create list: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateList,
        user_name: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        let list = ListService::get_by_user_and_listname(pool, user_name, listname.clone()).await?;
        let original_name = listname;
        let list_name = update_obj.list_name.unwrap_or(list.listname);
        let list_type = update_obj.list_type.unwrap_or(list.listtype);
        sqlx::query_as!(List, r#"UPDATE Lists SET username = $1, listname = $3, listtype = $4 WHERE username = $1 AND listname = $2 RETURNING username, listname, listtype AS "listtype: ListType""#, list.username, original_name, list_name, list_type as ListType).fetch_one(pool).await.map_err(|e| ErrorList(format!("failed to update list: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        user_name: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        sqlx::query_as!(
            List,
            r#"DELETE FROM Lists WHERE username = $1 AND listName = $2 RETURNING username, listname, listtype AS "listtype: ListType""#,
            user_name,
            listname
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorList(format!("failed to delete list: {e:#?}")))
    }
}
