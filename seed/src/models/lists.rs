use serde::Deserialize;

use super::{list_type::ListType, traits::Commit};

#[derive(Debug, Deserialize)]
pub struct List {
    pub user_name: String,
    pub list_name: String,
    pub list_type: ListType,
}

impl Commit for List {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_names = values
            .iter()
            .map(|l| l.user_name.clone())
            .collect::<Vec<String>>();
        let list_names = values
            .iter()
            .map(|l| l.list_name.clone())
            .collect::<Vec<String>>();
        let list_types = values
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
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ListItem {
    pub user_name: String,
    pub list_name: String,
    pub rank_in_list: i32,
    pub item_id: i32,
}

impl Commit for ListItem {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_names = values
            .iter()
            .map(|l| l.user_name.clone())
            .collect::<Vec<String>>();
        let list_names = values
            .iter()
            .map(|l| l.list_name.clone())
            .collect::<Vec<String>>();
        let rank_in_lists = values
            .iter()
            .map(|l| l.rank_in_list as i64)
            .collect::<Vec<i64>>();
        let item_ids = values
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
        Ok(())
    }
}
