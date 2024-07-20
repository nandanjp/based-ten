use serde::Deserialize;

pub trait Commit {
    type Value: Sized + for<'de> Deserialize<'de>;
    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
