use crate::models::listitems::{CreateListItem, ErrorListItem, ListItem, UpdateListItem};

pub struct ListItemService;

impl ListItemService {
    pub async fn get_list_item(
        pool: &sqlx::PgPool,
        user_name: String,
        listname: String,
        item_id: i32,
    ) -> Result<ListItem, ErrorListItem> {
        sqlx::query_as!(
            ListItem,
            "SELECT * FROM ListItems WHERE userName = $1 AND listName = $2 AND itemID = $3",
            user_name,
            listname,
            item_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorListItem(format!("failed to retrieve list item: {e:#?}")))
    }

    pub async fn create_list_item(
        pool: &sqlx::PgPool,
        create_obj: CreateListItem,
    ) -> Result<ListItem, ErrorListItem> {
        sqlx::query_as!(ListItem, "INSERT INTO ListItems(userName, listName, rankingInList, itemID) VALUES($1, $2, $3, $4) RETURNING *", create_obj.user_name, create_obj.list_name, create_obj.ranking_in_list, create_obj.item_id).fetch_one(pool).await.map_err(|e| ErrorListItem(format!("failed to create a list item: {e:#?}")))
    }
    pub async fn update_list_item(
        pool: &sqlx::PgPool,
        update: UpdateListItem,
        user_name: String,
        listname: String,
        item_id: i32,
    ) -> Result<ListItem, ErrorListItem> {
        let list_item =
            Self::get_list_item(&pool, user_name.clone(), listname.clone(), item_id).await?;
        let user_name = update.user_name.unwrap_or(list_item.username);
        let list_name = update.list_name.unwrap_or(list_item.listname);
        let ranking_in_list = update.ranking_in_list.unwrap_or(list_item.rankinginlist);
        let item_id = update.item_id;
        sqlx::query_as!(ListItem, "UPDATE ListItems SET rankingInList = $1 WHERE userName = $2 AND listName = $3 AND itemID = $4 RETURNING *", ranking_in_list, user_name, list_name, item_id).fetch_one(pool).await.map_err(|e| ErrorListItem(format!("failed to update list item: {e:#?}")))
    }
    pub async fn delete_list_item(
        pool: &sqlx::PgPool,
        user_name: String,
        listname: String,
        item_id: i32,
    ) -> Result<ListItem, ErrorListItem> {
        sqlx::query_as!(
            ListItem, "DELETE FROM ListItems WHERE userName = $1 AND listName = $2 AND itemID = $3 RETURNING *",
            user_name,
            listname,
            item_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            ErrorListItem(format!(
                "failed to delete list item due to the following error: {e:#?}"
            ))
        })
    }
}
