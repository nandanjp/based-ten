use crate::models::media::{Media, MediaError};

pub struct MediaService;
impl MediaService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Media>, MediaError> {
        sqlx::query!(
            r#"SELECT * FROM Media"#
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
        })
    }
}