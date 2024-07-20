use serde::Serialize;

use crate::{
    models::{
        listitems::ErrorListItem,
        lists::{CreateList, ErrorList, List, ListType, ListWithLikes, QueryList, UpdateList},
    },
    utils::traits::IntoSerial,
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

impl IntoSerial for FullListItem {
    type Serial = Self;

    fn to_serial(&self) -> Self::Serial {
        Self {
            user_name: self.user_name.clone(),
            list_name: self.list_name.clone(),
            ranking_in_list: self.ranking_in_list,
            item_id: self.item_id,
            list_type: self.list_type,
            likes: self.likes
        }
    }
}

pub struct ListService;
impl ListService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryList,
    ) -> Result<Vec<ListWithLikes>, ErrorList> {
        match query_obj {
            _ => sqlx::query!(
                r#"
                SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
                FROM Lists JOIN Likes ON Lists.listname = Likes.listname
                GROUP BY Lists.username, Lists.listname, listtype
                "#
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| ListWithLikes {
                        user_name: a.username,
                        list_name: a.listname,
                        list_type: a.listtype,
                        likes: a.likes.unwrap_or_default()
                    })
                    .collect::<Vec<ListWithLikes>>()
            })
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve all lists due to the following error: {e:#?}"
                ))
            }),
        }
    }

    pub async fn get_by_email(
        pool: &sqlx::PgPool,
        user_name: String,
    ) -> Result<Vec<ListWithLikes>, ErrorList> {
        sqlx::query!(r#"
            SELECT Lists.username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
            FROM Lists JOIN Likes ON Lists.listname = Likes.listname
            WHERE username = $1
            GROUP BY Lists.username, Lists.listname, listtype
        "#, user_name)
            .fetch_all(pool)
            .await
            .map(|a| a.into_iter().map(|a| ListWithLikes {
                user_name: a.username,
                list_name: a.listname,
                list_type: a.listtype,
                likes: a.likes.unwrap_or_default()
            }).collect::<Vec<ListWithLikes>>())
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list with user_name = {user_name} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn get_by_list_and_items_by_name(
        pool: &sqlx::PgPool,
        list_name: String,
    ) -> Result<Vec<FullListItem>, ErrorList> {
        sqlx::query!(r#"
            SELECT Lists.username, Lists.listname, rankinginlist, itemid, listtype AS "listtype: ListType", COUNT(*) as likes 
            FROM Lists JOIN ListItems ON Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname 
            WHERE Lists.listname = $1
            GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, listtype
        "#, list_name)
        .fetch_all(pool)
        .await
        .map(|a| 
            a.into_iter()
            .map(|a| FullListItem {
                item_id: a.itemid,
                list_name: a.listname,
                list_type: a.listtype,
                ranking_in_list: a.rankinginlist,
                user_name: a.username,
                likes: a.likes.unwrap_or_default()
            }).collect::<Vec<FullListItem>>())
            .map_err(|e| ErrorList(format!("failed to retrieve list with the given list name due to the following error: {e:#?}")))
    }

    pub async fn get_by_user_and_listname(
        pool: &sqlx::PgPool,
        user_name: String,
        list_name: String,
    ) -> Result<ListWithLikes, ErrorList> {
        sqlx::query!(r#"
            SELECT username, Lists.listname, listtype AS "listtype: ListType", COUNT(*) as likes
            FROM Lists JOIN Likes ON Lists.listname = Likes.listname
            WHERE username = $1 AND Lists.listname = $2
            GROUP BY username, Lists.listname, listtype
        "#, user_name, list_name)
            .fetch_one(pool)
            .await
            .map(|a| ListWithLikes {
                user_name: a.username,
                list_name: a.listname,
                list_type: a.listtype,
                likes: a.likes.unwrap_or_default()
            })
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list with user_name = {user_name} and listname = {list_name} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn get_user_list_and_items(
        pool: &sqlx::PgPool,
        user_name: String,
        list_name: String,
    ) -> Result<Vec<FullListItem>, ErrorListItem> {
        sqlx::query!(r#"
            SELECT Lists.username, Lists.listname, rankinginlist, itemid, listtype AS "listtype: ListType", COUNT(*) as likes
            FROM Lists JOIN ListItems ON Lists.username = ListItems.username AND Lists.listname = ListItems.listname JOIN Likes ON Lists.listname = Likes.listname
            WHERE Lists.username = $1 AND Lists.listname = $2
            GROUP BY Lists.username, Lists.listname, rankinginlist, itemid, listtype
            ORDER BY rankinginlist"#, user_name, list_name
        )
        .fetch_all(pool).await
        .map(|a| a.into_iter().map(|a| FullListItem {
                user_name: a.username,
                list_name: a.listname,
                ranking_in_list: a.rankinginlist,
                item_id: a.itemid,
                list_type: a.listtype,
                likes: a.likes.unwrap_or_default()
            }).collect::<Vec<FullListItem>>()
        )
        .map_err(|e| ErrorListItem(format!("failed to retrieve items of the list with list_name = {list_name}, user_name = {user_name} due to the following error: {e:#?}")))
    }

    pub async fn create(pool: &sqlx::PgPool, create_obj: CreateList) -> Result<List, ErrorList> {
        sqlx::query!(r#"INSERT INTO Lists(userName, listName, listType) VALUES($1, $2, $3) RETURNING userName, listName, listtype AS "listtype: ListType""#, create_obj.user_name, create_obj.list_name, create_obj.list_type as ListType).fetch_one(pool).await.map(|a| List {
            user_name: a.username,
            list_name: a.listname,
            list_type: a.listtype,
        }).map_err(|e| ErrorList(format!("failed to create a list with the given values due to the following error: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateList,
        user_name: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        let list = ListService::get_by_user_and_listname(pool, user_name, listname.clone()).await?;
        let original_name = listname;
        let list_name = update_obj.list_name.unwrap_or(list.list_name);
        let list_type = update_obj.list_type.unwrap_or(list.list_type);
        sqlx::query!(r#"UPDATE Lists SET username = $1, listname = $3, listtype = $4 WHERE username = $1 AND listname = $2 RETURNING username, listName, listtype AS "listtype: ListType""#, list.user_name, original_name, list_name, list_type as ListType).fetch_one(pool).await.map(|a| List {
            user_name: a.username,
            list_name: a.listname,
            list_type: a.listtype as ListType,
        }).map_err(|e| ErrorList(format!("failed to update list due to the following error: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        user_name: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        sqlx::query!(
            r#"DELETE FROM Lists WHERE username = $1 AND listName = $2 RETURNING userName, listName, listtype AS "listtype: ListType""#,
            user_name,
            listname
        )
        .fetch_one(pool)
        .await
        .map(|a| List {
            user_name: a.username,
            list_name: a.listname,
            list_type: a.listtype,
        })
        .map_err(|e| {
            ErrorList(format!(
                "failed to delete list due to the following error: {e:#?}"
            ))
        })
    }
}
