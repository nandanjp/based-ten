use crate::models::listitems::{ErrorListItem, ListItem, CreateListItem, UpdateListItem};

pub struct ListItemService;

impl ListItemService {
    pub async fn get_list_item(pool: &sqlx::PgPool, email: String, listname: String, item_id: i32) -> Result<ListItem, ErrorListItem> {
        sqlx::query!("SELECT * FROM ListItems WHERE email = $1 AND listName = $2 AND itemID = $3", email, listname, item_id)
            .fetch_one(pool)
            .await
            .map(|a| ListItem {
                email: a.email,
                list_name: a.listname,
                ranking_in_list: a.rankinginlist,
                item_id: a.itemid,
            })
            .map_err(|e| {
                ErrorListItem(format!(
                    "failed to retrieve list item with email = {email}, list_name = {listname}, item_id = {item_id} due to the following error: {e:#?}"
                ))
            })
    }
    pub async fn create_list_item(pool: &sqlx::PgPool, create_obj: CreateListItem) -> Result<ListItem, ErrorListItem> {
        sqlx::query!("INSERT INTO ListItems(email, listName, rankingInList, itemID) VALUES($1, $2, $3, $4) RETURNING email, listName, rankingInList, itemID", create_obj.email, create_obj.list_name, create_obj.ranking_in_list, create_obj.item_id).fetch_one(pool).await.map(|a| ListItem {
            email: a.email,
            list_name: a.listname,
            ranking_in_list: a.rankinginlist,
            item_id: a.itemid
        }).map_err(|e| ErrorListItem(format!("failed to create a list item with the given values due to the following error: {e:#?}")))
    }
    pub async fn update_list_item(
        pool: &sqlx::PgPool,
        update: UpdateListItem,
        email: String,
        listname: String,
        item_id: i32,
    ) -> Result<ListItem, ErrorListItem> {
        let list_item = Self::get_list_item(&pool, email.clone(), listname.clone(), item_id).await?;
        let email = update.email.unwrap_or(list_item.email);
        let list_name = update.list_name.unwrap_or(list_item.list_name);
        let ranking_in_list = update.ranking_in_list.unwrap_or(list_item.ranking_in_list);
        let item_id = update.item_id;
        sqlx::query!("UPDATE ListItems SET rankingInList = $1 WHERE email = $2 AND listName = $3 AND itemID = $4 RETURNING email, listName, rankingInList, itemID", ranking_in_list, email, list_name, item_id).fetch_one(pool).await.map(|a| ListItem {
            email: a.email,
            list_name: a.listname,
            ranking_in_list: a.rankinginlist,
            item_id: a.itemid
        }).map_err(|e| ErrorListItem(format!("failed to update list item due to the following error: {e:#?}")))
    }
    pub async fn delete_list_item(pool: &sqlx::PgPool, email: String, listname: String, item_id: i32) -> Result<ListItem, ErrorListItem> {
        sqlx::query!(
            "DELETE FROM ListItems WHERE email = $1 AND listName = $2 AND itemID = $3 RETURNING email, listName, rankingInList, itemID",
            email,
            listname,
            item_id
        )
        .fetch_one(pool)
        .await
        .map(|a| ListItem {
            email: a.email,
            list_name: a.listname,
            ranking_in_list: a.rankinginlist,
            item_id: a.itemid,
        })
        .map_err(|e| {
            ErrorListItem(format!(
                "failed to delete list item due to the following error: {e:#?}"
            ))
        })
    }
}
