use crate::models::{
    groups::{CreateGroups, Group, GroupMember, GroupRecursive, GroupsError, QueryGroups},
    lists::{List, ListType},
};

pub struct GroupsService;
impl GroupsService {
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Group>, GroupsError> {
        sqlx::query_as!(Group, r#"SELECT * FROM Groups"#,)
            .fetch_all(pool)
            .await
            .map_err(|e| GroupsError(format!("failed to retrieve all groups: {e:#?}")))
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, gid: i32) -> Result<Group, GroupsError> {
        sqlx::query_as!(Group, r#"SELECT * FROM Groups WHERE gid = $1"#, gid)
            .fetch_one(pool)
            .await
            .map_err(|e| GroupsError(format!("failed to retrieve group = {gid}: {e:#?}")))
    }

    pub async fn get_user_groups(
        pool: &sqlx::PgPool,
        username: String,
    ) -> Result<Vec<Group>, GroupsError> {
        sqlx::query_as!(
            Group,
            r#"SELECT gm.gid AS gid, g.groupName AS groupName, g.ownedBy AS ownedBy 
            FROM GroupMembers gm JOIN Groups g ON gm.gid = g.gid
            WHERE gm.username = $1"#,
            username
        )
        .fetch_all(pool)
        .await
        .map_err(|e| GroupsError(format!("failed to retrieve user's groups: {e:#?}")))
    }

    pub async fn get_circles_by_id(
        pool: &sqlx::PgPool,
        gid: i32,
    ) -> Result<Vec<GroupRecursive>, GroupsError> {
        sqlx::query_as!(
            GroupRecursive,
            r#"WITH RECURSIVE Circles AS (
                    (SELECT g.gid, g.groupName, g.ownedBy, 1 AS level
                    FROM Groups g
                    WHERE g.gid = $1)
                    UNION
                    (
                        SELECT gm2.gid, g.groupName, g.ownedBy, 1 + level AS level
                        FROM Circles c JOIN GroupMembers gm1 ON c.gid = gm1.gid
                        JOIN GroupMembers gm2 ON gm1.username = gm2.username
                        JOIN Groups g ON gm2.gid = g.gid
                        WHERE level < 3
                    )
                )
                SELECT DISTINCT gid, groupName, ownedBy FROM Circles
                ORDER BY gid"#,
            gid
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            GroupsError(format!(
                "failed to retrieve a group with gid = {gid}: {e:#?}"
            ))
        })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        username: String,
        create_obj: CreateGroups,
    ) -> Result<Group, GroupsError> {
        sqlx::query_as!(
            Group,
            r#"INSERT INTO Groups(gid, groupName, ownedBy) VALUES($1, $2, $3) RETURNING *"#,
            create_obj.gid,
            create_obj.group_name,
            username
        )
        .fetch_one(pool)
        .await
        .map_err(|e| GroupsError(format!("failed to create group: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, gid: i32) -> Result<Group, GroupsError> {
        sqlx::query_as!(
            Group,
            r#"DELETE FROM Groups WHERE gid = $1 RETURNING *"#,
            gid
        )
        .fetch_one(pool)
        .await
        .map_err(|e| GroupsError(format!("failed to delete group = {gid}: {e:#?}")))
    }

    pub async fn delete_user_group(
        pool: &sqlx::PgPool,
        username: String,
        group_name: String,
    ) -> Result<Group, GroupsError> {
        sqlx::query_as!(
            Group,
            r#"DELETE FROM Groups WHERE ownedby = $1 AND groupname = $2 RETURNING *"#,
            username,
            group_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| GroupsError(format!("failed to delete user's group: {e:#?}")))
    }

    pub async fn get_member_lists(
        pool: &sqlx::PgPool,
        gid: i32,
        order_by_author: QueryGroups,
    ) -> Result<Vec<List>, GroupsError> {
        match order_by_author {
            QueryGroups {
                order_by_author: Some(true),
            } => sqlx::query_as!(
                List,
                r#"SELECT l.listName, listType AS "listtype: ListType", l.username
                    FROM (GroupMembers m JOIN Users u ON m.username = u.username) JOIN Lists l
                    ON u.username = l.username
                    WHERE m.gid = $1
                    ORDER BY l.username"#,
                gid
            )
            .fetch_all(pool)
            .await
            .map_err(|e| GroupsError(format!("failed to get group members {gid}: {e:#?}"))),
            _ => sqlx::query_as!(
                List,
                r#"SELECT l.listName, listType AS "listtype: ListType", l.username
                    FROM (GroupMembers m JOIN Users u ON m.username = u.username) JOIN Lists l
                    ON u.username = l.username
                    WHERE m.gid = $1
                    ORDER BY listtype"#,
                gid
            )
            .fetch_all(pool)
            .await
            .map_err(|e| GroupsError(format!("failed to get group members of {gid}: {e:#?}"))),
        }
    }

    pub async fn get_members(
        pool: &sqlx::PgPool,
        gid: i32,
    ) -> Result<Vec<GroupMember>, GroupsError> {
        sqlx::query_as!(
            GroupMember,
            r#"SELECT username FROM GroupMembers WHERE gid = $1"#,
            gid
        )
        .fetch_all(pool)
        .await
        .map_err(|e| GroupsError(format!("failed to get group members of {gid}: {e:#?}")))
    }
}
