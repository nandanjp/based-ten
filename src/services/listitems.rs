use crate::models::listitems::{ErrorListItem, ListItem, UpdateListItem};

pub struct ListItemService;

impl ListItemService {
    async fn get_list_item(pool: &sqlx::PgPool) -> Result<ListItem, ErrorListItem> {}
    async fn create_list_item(pool: &sqlx::PgPool) -> Result<ListItem, ErrorListItem> {}
    async fn update_list_item(
        pool: &sqlx::PgPool,
        update: UpdateListItem,
    ) -> Result<ListItem, ErrorListItem> {
    }
    async fn delete_list_item(pool: &sqlx::PgPool, id: i32) -> Result<ListItem, ErrorListItem> {}
}
