use crate::models::{
    lists::ListType,
    media::{Media, MediaError, QueryMedia},
};

pub struct MediaService;
impl MediaService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryMedia,
    ) -> Result<Vec<Media>, MediaError> {
        match query_obj {
            QueryMedia {
                media_type: Some(media_type),
            } => sqlx::query!(
                r#"SELECT id, title, mediaimage, createdon, type AS "type: ListType" FROM Media WHERE type = $1"#, media_type as ListType
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Media {
                        id: a.id.unwrap(),
                        title: a.title.unwrap(),
                        media_image: a.mediaimage.unwrap(),
                        created_on: a.createdon.unwrap(),
                        media_type: a.r#type.unwrap(),
                    })
                    .collect::<Vec<Media>>()
            })
            .map_err(|e| {
                MediaError(format!(
                    "failed to retrieve all media due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(
                r#"SELECT id, title, mediaimage, createdon, type AS "type: ListType" FROM Media"#
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Media {
                        id: a.id.unwrap(),
                        title: a.title.unwrap(),
                        media_image: a.mediaimage.unwrap(),
                        created_on: a.createdon.unwrap(),
                        media_type: a.r#type.unwrap(),
                    })
                    .collect::<Vec<Media>>()
            })
            .map_err(|e| {
                MediaError(format!(
                    "failed to retrieve all media due to the following error: {e:#?}"
                ))
            }),
        }
    }
}