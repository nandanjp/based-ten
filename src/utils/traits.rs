use axum::async_trait;
use serde::Serialize;

#[async_trait]
pub trait GeneralService {
    type Response;
    type ListResponse;
    type QueryObject;
    type CreateObject;
    type UpdateObject;
    type Error;

    async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error>;
    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error>;
    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error>;
    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error>;
    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error>;
}

pub trait IntoSerial {
    type Serial: Serialize;
    fn to_serial(&self) -> Self::Serial;
}

pub trait Error {
    fn new(err: String) -> Self;
}
