use serde::Serialize;

use crate::models::{
    listitems::ErrorListItem,
    lists::{CreateList, ErrorList, List, ListType, QueryList, UpdateList},
};

#[derive(Debug, Clone, Serialize)]
pub struct FullListItem {
    pub email: String,
    pub list_name: String,
    pub ranking_in_list: i32,
    pub item_id: i32,
    #[serde(rename = "type")]
    pub list_type: ListType,
}

pub struct ListService;
impl ListService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryList,
    ) -> Result<Vec<List>, ErrorList> {
        match query_obj {
            _ => sqlx::query!(
                r#"SELECT email, listName, listtype AS "listtype: ListType" FROM Lists"#
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| List {
                        email: a.email,
                        list_name: a.listname,
                        list_type: a.listtype,
                    })
                    .collect::<Vec<List>>()
            })
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve all lists due to the following error: {e:#?}"
                ))
            }),
        }
    }

    pub async fn get_by_email(pool: &sqlx::PgPool, email: String) -> Result<Vec<List>, ErrorList> {
        sqlx::query!(r#"SELECT email, listName, listtype AS "listtype: ListType" FROM Lists WHERE email = $1"#, email)
            .fetch_all(pool)
            .await
            .map(|a| a.into_iter().map(|a| List {
                email: a.email,
                list_name: a.listname,
                list_type: a.listtype,
            }).collect::<Vec<List>>())
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list with email = {email} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn get_by_user_and_listname(
        pool: &sqlx::PgPool,
        email: String,
        list_name: String,
    ) -> Result<List, ErrorList> {
        sqlx::query!(r#"SELECT email, listname, listtype AS "listtype: ListType" FROM Lists WHERE email = $1 AND listname = $2"#, email, list_name)
            .fetch_one(pool)
            .await
            .map(|a| List {
                email: a.email,
                list_name: a.listname,
                list_type: a.listtype,
            })
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list with email = {email} and listname = {list_name} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn get_user_list_and_items(
        pool: &sqlx::PgPool,
        email: String,
        list_name: String,
    ) -> Result<Vec<FullListItem>, ErrorListItem> {
        sqlx::query!(r#"
            SELECT Lists.email, Lists.listname, rankinginlist, itemid, listtype AS "listtype: ListType" 
            FROM Lists JOIN ListItems ON Lists.email = ListItems.email AND Lists.listname = ListItems.listname 
            WHERE Lists.email = $1 AND Lists.listname = $2 
            ORDER BY rankinginlist"#, email, list_name
        )
        .fetch_all(pool).await
        .map(|a| a.into_iter().map(|a| FullListItem {
                email: a.email,
                list_name: a.listname,
                ranking_in_list: a.rankinginlist,
                item_id: a.itemid,
                list_type: a.listtype
            }).collect::<Vec<FullListItem>>()
        )
        .map_err(|e| ErrorListItem(format!("failed to retrieve items of the list with list_name = {list_name}, email = {email} due to the following error: {e:#?}")))
    }

    pub async fn create(pool: &sqlx::PgPool, create_obj: CreateList) -> Result<List, ErrorList> {
        sqlx::query!(r#"INSERT INTO Lists(email, listName, listType) VALUES($1, $2, $3) RETURNING email, listName, listtype AS "listtype: ListType""#, create_obj.email, create_obj.list_name, create_obj.list_type as ListType).fetch_one(pool).await.map(|a| List {
            email: a.email,
            list_name: a.listname,
            list_type: a.listtype
        }).map_err(|e| ErrorList(format!("failed to create a list with the given values due to the following error: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateList,
        email: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        let list = ListService::get_by_user_and_listname(pool, email, listname.clone()).await?;
        let original_name = listname;
        let list_name = update_obj.list_name.unwrap_or(list.list_name);
        let list_type = update_obj.list_type.unwrap_or(list.list_type);
        sqlx::query!(r#"UPDATE Lists SET email = $1, listname = $3, listtype = $4 WHERE email = $1 AND listname = $2 RETURNING email, listName, listtype AS "listtype: ListType""#, list.email, original_name, list_name, list_type as ListType).fetch_one(pool).await.map(|a| List {
            email: a.email,
            list_name: a.listname,
            list_type: a.listtype as ListType
        }).map_err(|e| ErrorList(format!("failed to update list due to the following error: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        email: String,
        listname: String,
    ) -> Result<List, ErrorList> {
        sqlx::query!(
            r#"DELETE FROM Lists WHERE email = $1 AND listName = $2 RETURNING email, listName, listtype AS "listtype: ListType""#,
            email,
            listname
        )
        .fetch_one(pool)
        .await
        .map(|a| List {
            email: a.email,
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
