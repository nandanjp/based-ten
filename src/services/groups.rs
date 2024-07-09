use crate::{
    models::groups::{CreateGroups, Groups, GroupsError, GroupsQuery, UpdateGroups},
    utils::traits::GeneralService,
};
use axum::async_trait;

pub struct GroupsService;
#[async_trait]
impl GeneralService for GroupsService {
    type Response = Groups;
    type ListResponse = Vec<Groups>;
    type QueryObject = GroupsQuery;
    type CreateObject = CreateGroups;
    type UpdateObject = UpdateGroups;
    type Error = GroupsError;

    async fn get_all(
        pool: &sqlx::PgPool,
        _query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error> {
        sqlx::query!(r#"SELECT * FROM Groups"#,)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Self::Response {
                        gid: a.gid,
                        group_name: a.groupname,
                        owned_by: a.ownedby,
                    })
                    .collect::<Self::ListResponse>()
            })
            .map_err(|e| {
                GroupsError(format!(
                    "failed to retrieve all groups due to the following error: {e:#?}"
                ))
            })
    }

    async fn get_by_id(pool: &sqlx::PgPool, gid: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"SELECT * FROM Groups WHERE gid = $1"#, gid)
            .fetch_one(pool)
            .await
            .map(|a| Self::Response {
                gid: a.gid,
                group_name: a.groupname,
                owned_by: a.ownedby,
            })
            .map_err(|e| {
                GroupsError(format!(
                    "failed to retrieve a group with gid = {gid} due to the following error: {e:#?}"
                ))
            })
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"INSERT INTO Groups(gid, groupName, ownedBy) VALUES($1, $2, $3) RETURNING gid, groupName, ownedBy"#, create_obj.gid, create_obj.group_name, create_obj.owned_by).fetch_one(pool).await.map(|a| Self::Response {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to create groups with the provided details due to the following error: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        gid: i32,
    ) -> Result<Self::Response, Self::Error> {
        let groups = Self::get_by_id(&pool, gid).await?;
        let group_name = update_obj.group_name.unwrap_or(groups.group_name);
        let owned_by = update_obj.owned_by.unwrap_or(groups.owned_by);

        sqlx::query!(r#"UPDATE Groups SET groupName = $1, ownedBy = $2 WHERE gid = $3 RETURNING gid, groupName, ownedBy"#, group_name, owned_by, gid).fetch_one(pool).await.map(|a| Self::Response {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to update groups with the provided details due to the following error: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, gid: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"DELETE FROM Groups WHERE gid = $1 RETURNING gid, groupName, ownedBy"#, gid).fetch_one(pool).await.map(|a| Self::Response {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to delete groups with the given gid = {gid} due to the following error: {e:#?}")))
    }
}
