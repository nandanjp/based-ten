use serde::Serialize;

use crate::models::lists::{
    CreateList, ErrorList, List, ListOrderedByLikes, ListType, ListWithLikes, QueryList, UpdateList,
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
                SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(Likes.likername) as likes
                FROM Lists LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname AND Lists.username = Likes.likingname
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
            SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(Likes.likername) as likes
            FROM Lists LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname AND Lists.username = Likes.likingname
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
            SELECT username, Lists.listname, listtype AS "listtype: ListType", COUNT(Likes.likername) as likes
            FROM Lists LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname AND Lists.username = Likes.likingname
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

    pub async fn get_user_list_type(
        pool: &sqlx::PgPool,
        username: String,
        list_name: String,
    ) -> Result<serde_json::Value, ErrorList> {
        sqlx::query!(
            r#"
            SELECT listtype AS "listtype: ListType" FROM Lists WHERE listname = $1 AND username = $2
        "#,
            list_name,
            username
        )
        .fetch_one(pool)
        .await
        .map(|r| {
            serde_json::json!({
                "listtype": r.listtype
            })
        })
        .map_err(|e| ErrorList(format!("failed to retrieve list of given type: {e:#?}")))
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
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, numepisodes, createdon, COUNT(Likes.likername) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname JOIN Anime ON ListItems.itemid = Anime.id
                    WHERE Lists.listname = $1 AND Lists.username = $2 AND ListItems.listname = $1 AND ListItems.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, numepisodes, createdon, listtype
                    ORDER BY rankinginlist
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
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, createdon, COUNT(Likes.likername) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname JOIN Movies ON ListItems.itemid = Movies.id
                    WHERE Lists.listname = $1 AND Lists.username = $2 AND ListItems.listname = $1 AND ListItems.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, createdon, listtype
                    ORDER BY rankinginlist
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
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, console, createdon, COUNT(Likes.likername) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname AND Lists.username = Likes.likingname JOIN VideoGames ON ListItems.itemid = VideoGames.id
                    WHERE Lists.listname = $1 AND Lists.username = $2 AND ListItems.listname = $1 AND ListItems.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, console, createdon, listtype
                    ORDER BY rankinginlist
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
                    SELECT Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, author, album, createdon, COUNT(Likes.likername) as likes, listtype AS "listtype: ListType"
                    FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname LEFT OUTER JOIN Likes ON Lists.listname = Likes.listname JOIN Songs ON ListItems.itemid = Songs.id
                    WHERE Lists.listname = $1 AND Lists.username = $2 AND ListItems.listname = $1 AND ListItems.username = $2
                    GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, title, mediaimage, author, album, createdon, listtype
                    ORDER BY rankinginlist
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

    pub async fn get_lists_ordered_likes(
        pool: &sqlx::PgPool,
        list_type: ListType,
    ) -> Result<Vec<ListOrderedByLikes>, ErrorList> {
        match list_type {
            ListType::Anime => sqlx::query_as!(
                ListOrderedByLikes,
                r#"WITH ListsWithItemIDs AS (
                    SELECT l.userName,
                        l.listName,
                        li.itemID,
                        l.listType as itemType
                    FROM ListItems li
                        JOIN Lists l ON li.userName = l.userName
                        AND li.listName = l.listName
                ),
                ListsWithLikes AS (
                    SELECT l.*
                    FROM Lists l
                        JOIN Likes lk on l.listName = lk.listName
                ),
                ListLikeCounts AS (
                    SELECT lwl.userName,
                        lwl.listName,
                        COUNT(*) as likeCount
                    FROM ListsWithLikes lwl
                    GROUP BY lwl.userName,
                        lwl.listName
                ),
                TotalLikesByItem AS (
                    SELECT lwi.itemID,
                        lwi.itemType,
                        CAST(SUM(llc.likeCount) AS INTEGER) as totalLikes
                    FROM ListsWithItemIDs lwi
                        JOIN ListLikeCounts llc ON lwi.userName = llc.userName
                        AND lwi.listName = llc.listName
                    GROUP BY lwi.itemID,
                        lwi.itemType
                )
                SELECT id,
                    title,
                    mediaimage,
                    createdon,
                    type AS "listtype: ListType",
                    totalLikes
                FROM Media m
                    JOIN TotalLikesByItem l ON m.id = l.itemID
                    AND m.type = l.itemType
                WHERE m.type = 'anime'
                ORDER BY l.totalLikes DESC
            "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorList(format!("failed to retrieve explore lists: {e:#?}"))),
            ListType::Movies => sqlx::query_as!(
                ListOrderedByLikes,
                r#"WITH ListsWithItemIDs AS (
                    SELECT l.userName,
                        l.listName,
                        li.itemID,
                        l.listType as itemType
                    FROM ListItems li
                        JOIN Lists l ON li.userName = l.userName
                        AND li.listName = l.listName
                ),
                ListsWithLikes AS (
                    SELECT l.*
                    FROM Lists l
                        JOIN Likes lk on l.listName = lk.listName
                ),
                ListLikeCounts AS (
                    SELECT lwl.userName,
                        lwl.listName,
                        COUNT(*) as likeCount
                    FROM ListsWithLikes lwl
                    GROUP BY lwl.userName,
                        lwl.listName
                ),
                TotalLikesByItem AS (
                    SELECT lwi.itemID,
                        lwi.itemType,
                        CAST(SUM(llc.likeCount) AS INTEGER) as totalLikes
                    FROM ListsWithItemIDs lwi
                        JOIN ListLikeCounts llc ON lwi.userName = llc.userName
                        AND lwi.listName = llc.listName
                    GROUP BY lwi.itemID,
                        lwi.itemType
                )
                SELECT id,
                    title,
                    mediaimage,
                    createdon,
                    type AS "listtype: ListType",
                    totalLikes
                FROM Media m
                    JOIN TotalLikesByItem l ON m.id = l.itemID
                    AND m.type = l.itemType
                WHERE m.type = 'movies'
                ORDER BY l.totalLikes DESC
            "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorList(format!("failed to retrieve explore lists: {e:#?}"))),
            ListType::Songs => sqlx::query_as!(
                ListOrderedByLikes,
                r#"WITH ListsWithItemIDs AS (
                    SELECT l.userName,
                        l.listName,
                        li.itemID,
                        l.listType as itemType
                    FROM ListItems li
                        JOIN Lists l ON li.userName = l.userName
                        AND li.listName = l.listName
                ),
                ListsWithLikes AS (
                    SELECT l.*
                    FROM Lists l
                        JOIN Likes lk on l.listName = lk.listName
                ),
                ListLikeCounts AS (
                    SELECT lwl.userName,
                        lwl.listName,
                        COUNT(*) as likeCount
                    FROM ListsWithLikes lwl
                    GROUP BY lwl.userName,
                        lwl.listName
                ),
                TotalLikesByItem AS (
                    SELECT lwi.itemID,
                        lwi.itemType,
                        CAST(SUM(llc.likeCount) AS INTEGER) as totalLikes
                    FROM ListsWithItemIDs lwi
                        JOIN ListLikeCounts llc ON lwi.userName = llc.userName
                        AND lwi.listName = llc.listName
                    GROUP BY lwi.itemID,
                        lwi.itemType
                )
                SELECT id,
                    title,
                    mediaimage,
                    createdon,
                    type AS "listtype: ListType",
                    totalLikes
                FROM Media m
                    JOIN TotalLikesByItem l ON m.id = l.itemID
                    AND m.type = l.itemType
                WHERE m.type = 'songs'
                ORDER BY l.totalLikes DESC
            "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorList(format!("failed to retrieve explore lists: {e:#?}"))),
            ListType::VideoGames => sqlx::query_as!(
                ListOrderedByLikes,
                r#"WITH ListsWithItemIDs AS (
                    SELECT l.userName,
                        l.listName,
                        li.itemID,
                        l.listType as itemType
                    FROM ListItems li
                        JOIN Lists l ON li.userName = l.userName
                        AND li.listName = l.listName
                ),
                ListsWithLikes AS (
                    SELECT l.*
                    FROM Lists l
                        JOIN Likes lk on l.listName = lk.listName
                ),
                ListLikeCounts AS (
                    SELECT lwl.userName,
                        lwl.listName,
                        COUNT(*) as likeCount
                    FROM ListsWithLikes lwl
                    GROUP BY lwl.userName,
                        lwl.listName
                ),
                TotalLikesByItem AS (
                    SELECT lwi.itemID,
                        lwi.itemType,
                        CAST(SUM(llc.likeCount) AS INTEGER) as totalLikes
                    FROM ListsWithItemIDs lwi
                        JOIN ListLikeCounts llc ON lwi.userName = llc.userName
                        AND lwi.listName = llc.listName
                    GROUP BY lwi.itemID,
                        lwi.itemType
                )
                SELECT id,
                    title,
                    mediaimage,
                    createdon,
                    type AS "listtype: ListType",
                    totallikes
                FROM Media m
                    JOIN TotalLikesByItem l ON m.id = l.itemID
                    AND m.type = l.itemType
                WHERE m.type = 'videogames'
                ORDER BY l.totalLikes DESC
            "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorList(format!("failed to retrieve explore lists: {e:#?}"))),
        }
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

    pub async fn create(
        pool: &sqlx::PgPool,
        user_name: String,
        create_obj: CreateList,
    ) -> Result<List, ErrorList> {
        let res = sqlx::query_as!(
            List,
            r#"INSERT INTO Lists(userName, listName, listType) VALUES($1, $2, $3) RETURNING username, listname, listtype AS "listtype: ListType""#,
            user_name,
            create_obj.list_name,
            create_obj.list_type as ListType
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorList(format!("failed to create list: {e:#?}")))?;

        let user_names = create_obj
            .list_items
            .iter()
            .map(|i| i.username.clone())
            .collect::<Vec<String>>();
        let list_names = create_obj
            .list_items
            .iter()
            .map(|i| i.listname.clone())
            .collect::<Vec<String>>();
        let rank_in_lists = create_obj
            .list_items
            .iter()
            .map(|i| i.rankinginlist as i64)
            .collect::<Vec<i64>>();
        let item_ids = create_obj
            .list_items
            .into_iter()
            .map(|i| i.itemid as i64)
            .collect::<Vec<i64>>();

        println!("User names: {:#?}", user_names);
        println!("List names: {:#?}", list_names);
        println!("Rank in lists: {:#?}", rank_in_lists);
        println!("Item ids: {:#?}", item_ids);

        sqlx::query!(
            r#"INSERT INTO ListItems(username, listname, rankinginlist, itemid) SELECT * FROM UNNEST($1::text[], $2::text[], $3::int8[], $4::int8[])"#,
            &user_names[..],
            &list_names[..],
            &rank_in_lists[..],
            &item_ids[..]
        )
        .execute(pool)
        .await
        .map_err(|e| {
            println!("SQL Error: {:#?}", e);
            ErrorList(format!("failed to add list item to list: {e:#?}"))
        })?;
        println!("List items added successfully");
        return Ok(res);
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
