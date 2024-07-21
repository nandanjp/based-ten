use crate::{
    models::groups::{CreateGroups, Group, GroupMember, GroupsError, QueryGroups, UpdateGroups},
    models::lists::{List, ListType},
};

pub struct GroupsService;
impl GroupsService {
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Group>, GroupsError> {
        sqlx::query!(r#"SELECT * FROM Groups"#,)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Group {
                        gid: a.gid,
                        group_name: a.groupname,
                        owned_by: a.ownedby,
                    })
                    .collect::<Vec<Group>>()
            })
            .map_err(|e| {
                GroupsError(format!(
                    "failed to retrieve all groups due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, gid: i32) -> Result<Group, GroupsError> {
        sqlx::query!(r#"SELECT * FROM Groups WHERE gid = $1"#, gid)
            .fetch_one(pool)
            .await
            .map(|a| Group {
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

    pub async fn get_circles_by_id(pool: &sqlx::PgPool, gid: i32) -> Result<Vec<Group>, GroupsError> {
        sqlx::query!(r#"WITH RECURSIVE Circles AS (
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
                    WHERE gid != $1
                    ORDER BY gid"#, gid)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Group {
                        gid: a.gid.unwrap(),
                        group_name: a.groupname.unwrap(),
                        owned_by: a.ownedby.unwrap(),
                    })
                    .collect::<Vec<Group>>()
            })
            .map_err(|e| {
                GroupsError(format!(
                    "failed to retrieve a group with gid = {gid} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateGroups,
    ) -> Result<Group, GroupsError> {
        sqlx::query!(r#"INSERT INTO Groups(gid, groupName, ownedBy) VALUES($1, $2, $3) RETURNING gid, groupName, ownedBy"#, create_obj.gid, create_obj.group_name, create_obj.owned_by).fetch_one(pool).await.map(|a| Group {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to create groups with the provided details due to the following error: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateGroups,
        gid: i32,
    ) -> Result<Group, GroupsError> {
        let groups = Self::get_by_id(&pool, gid).await?;
        let group_name = update_obj.group_name.unwrap_or(groups.group_name);
        let owned_by = update_obj.owned_by.unwrap_or(groups.owned_by);

        sqlx::query!(r#"UPDATE Groups SET groupName = $1, ownedBy = $2 WHERE gid = $3 RETURNING gid, groupName, ownedBy"#, group_name, owned_by, gid).fetch_one(pool).await.map(|a| Group {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to update groups with the provided details due to the following GroupsError: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, gid: i32) -> Result<Group, GroupsError> {
        sqlx::query!(r#"DELETE FROM Groups WHERE gid = $1 RETURNING gid, groupName, ownedBy"#, gid).fetch_one(pool).await.map(|a| Group {
            gid: a.gid,
            group_name: a.groupname,
            owned_by: a.ownedby
        }).map_err(|e| GroupsError(format!("failed to delete groups with the given gid = {gid} due to the following error: {e:#?}")))
    }

    pub async fn get_member_lists(
        pool: &sqlx::PgPool,
        gid: i32,
        order_by_author: QueryGroups,
    ) -> Result<Vec<List>, GroupsError> {
        match order_by_author {
            QueryGroups{order_by_author: Some(true)} =>
            sqlx::query!(r#"SELECT l.listName, listType AS "listtype: ListType", l.username
            FROM (GroupMembers m JOIN Users u ON m.username = u.username) JOIN Lists l
            ON u.username = l.username
            WHERE m.gid = $1
            ORDER BY l.username"#, gid).fetch_all(pool).await
            .map(|a| a.into_iter().map(|a| List {
                user_name: a.username,
                list_name: a.listname,
                list_type: a.listtype,
            }).collect::<Vec<List>>())
            .map_err(|e| GroupsError(format!("failed to get lists of group members for group with gid = {gid} due to the following error: {e:#?}"))),
            _ => {
                sqlx::query!(r#"SELECT l.listName, listType AS "listtype: ListType", l.username
                FROM (GroupMembers m JOIN Users u ON m.username = u.username) JOIN Lists l
                ON u.username = l.username
                WHERE m.gid = $1
                ORDER BY listtype"#, gid).fetch_all(pool).await
                .map(|a| a.into_iter().map(|a| List {
                    user_name: a.username,
                    list_name: a.listname,
                    list_type: a.listtype,
                }).collect::<Vec<List>>())
                .map_err(|e| GroupsError(format!("failed to get lists of group members for group with gid = {gid} due to the following error: {e:#?}")))
            }
        }
    }

    pub async fn get_members(
        pool: &sqlx::PgPool,
        gid: i32,
    ) -> Result<Vec<GroupMember>, GroupsError> {
        sqlx::query!(r#"SELECT username FROM GroupMembers WHERE gid = $1"#, gid)
        .fetch_all(pool).await
        .map(|a| a.into_iter().map(|a| GroupMember {
            user_name: a.username
        }).collect::<Vec<GroupMember>>())
        .map_err(|e| GroupsError(format!("failed to get group members for group with gid = {gid} due to the following error: {e:#?}")))
    }
}
