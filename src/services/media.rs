use crate::{
    models::{
        lists::ListType,
        media::{Media, MediaError, MediaSortKey, QueryMedia},
    },
    utils::functions::page_limit,
};

const NUM_ROWS: i64 = 12341;

pub struct MediaService;
impl MediaService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryMedia,
    ) -> Result<Vec<Media>, MediaError> {
        let (limit, page) = page_limit(NUM_ROWS, query_obj.limit, query_obj.page);

        match query_obj {
            QueryMedia {
                media_type: Some(media_type), sort_key: Some(key), ..
            } => match key {
                MediaSortKey::Title => sqlx::query_as!(Media,
                    r#"SELECT id, title, mediaimage, createdon, type AS "typ: ListType" FROM Media WHERE type = $1 ORDER BY title OFFSET $2 LIMIT $3"#, media_type as ListType, page * limit, limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| MediaError(format!("failed to retrieve all media: {e:#?}"))),
                MediaSortKey::Type => sqlx::query_as!(Media,
                    r#"SELECT id, title, mediaimage, createdon, type AS "typ: ListType" FROM Media WHERE type = $1 ORDER BY type OFFSET $2 LIMIT $3"#, media_type as ListType, page * limit, limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| MediaError(format!("failed to retrieve all media: {e:#?}")))
            },
            QueryMedia {
                sort_key: Some(key), ..
            } => match key {
                MediaSortKey::Title => sqlx::query_as!(Media,
                    r#"SELECT id, title, mediaimage, createdon, type AS "typ: ListType" FROM Media ORDER BY title OFFSET $1 LIMIT $2"#, page * limit, limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| MediaError(format!("failed to retrieve all media: {e:#?}"))),
                MediaSortKey::Type => sqlx::query_as!(Media,
                    r#"SELECT id, title, mediaimage, createdon, type AS "typ: ListType" FROM Media ORDER BY type OFFSET $1 LIMIT $2"#, page * limit, limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| MediaError(format!("failed to retrieve all media: {e:#?}")))
            },
            _ => sqlx::query_as!(Media,
                r#"SELECT id, title, mediaimage, createdon, type AS "typ: ListType" FROM Media OFFSET $1 LIMIT $2"#, page * limit, limit
            )
            .fetch_all(pool)
            .await
            .map_err(|e| MediaError(format!("failed to retrieve all media: {e:#?}"))),
        }
    }
}
