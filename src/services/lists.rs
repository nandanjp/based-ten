use crate::{
    models::lists::{CreateList, ErrorList, List, QueryList, UpdateList, ListType},
    utils::{functions::convert_date, traits::GeneralService},
};
use axum::{async_trait, Error};

pub struct ListService;
impl ListService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryList,
    ) -> Result<Vec<List>, ErrorList> {
        match query_obj {
            _ => sqlx::query!(r#"SELECT email, listName, listType::text AS listtype FROM Lists"#)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| List {
                            email: a.email,
                            list_name: a.listname,
                            list_type: a.listtype.unwrap().parse::<ListType>().unwrap(),
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

    pub async fn get_by_id(pool: &sqlx::PgPool, email: String, listname: String) -> Result<List, ErrorList> {
        sqlx::query!("SELECT email, listName, listType::text AS listtype FROM Lists WHERE email = $1 AND listName = $2", email, listname)
            .fetch_one(pool)
            .await
            .map(|a| List {
                email: a.email,
                list_name: a.listname,
                list_type: a.listtype.unwrap().parse::<ListType>().unwrap(),
            })
            .map_err(|e| {
                ErrorList(format!(
                    "failed to retrieve list with email = {email} and listname = {listname} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateList,
    ) -> Result<List, ErrorList> {
        sqlx::query!("INSERT INTO Lists(email, listName, listType) VALUES($1, $2, $3) RETURNING email, listName, listType::text AS listtype", create_obj.email, create_obj.list_name, create_obj.list_type.to_string()).fetch_one(pool).await.map(|a| List {
            email: a.email,
            list_name: a.listname,
            list_type: a.listtype.unwrap().parse::<ListType>().unwrap()
        }).map_err(|e| ErrorList(format!("failed to create a list with the given values due to the following error: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateList,
        email: String,
        listname: String
    ) -> Result<List, ErrorList> {
        let list = ListService::get_by_id(pool, email, listname).await?;
        let email = update_obj.email.unwrap_or(list.email);
        let list_name = update_obj.list_name.unwrap_or(list.list_name);
        let list_type = update_obj.list_type.unwrap_or(list.list_type);
        sqlx::query!("UPDATE Lists SET email = $1, listName = $2, listType = $3 WHERE email = $1 AND listName = $2 RETURNING email, listName, listType::text AS listtype", email, list_name, list_type).fetch_one(pool).await.map(|a| List {
            email: a.email,
            list_name: a.listname,
            list_type: a.listtype.unwrap().parse::<ListType>().unwrap()
        }).map_err(|e| ErrorList(format!("failed to update list due to the following error: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, email: String, listname: String) -> Result<List, ErrorList> {
        sqlx::query!(
            "DELETE FROM Lists WHERE email = $1 AND listName = $2 RETURNING email, listName, listType::text AS listtype",
            email,
            listname
        )
        .fetch_one(pool)
        .await
        .map(|a| List {
            email: a.email,
            list_name: a.listname,
            list_type: a.listtype.unwrap().parse::<ListType>().unwrap(),
        })
        .map_err(|e| {
            ErrorList(format!(
                "failed to delete list due to the following error: {e:#?}"
            ))
        })
    }
}
