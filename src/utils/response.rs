use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

use super::traits::IntoSerial;

#[derive(Debug, Serialize)]
struct Response<T>
where
    T: Serialize,
{
    success: bool,
    response: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListResponse<T>
where
    T: Serialize,
{
    success: bool,
    response: Option<Vec<T>>,
    error: Option<String>,
}

pub fn get_list_response<T>(
    v: Result<Vec<T>, String>,
    succ_code: StatusCode,
    err_code: StatusCode,
) -> impl IntoResponse
where
    T: IntoSerial,
    T::Serial: Serialize,
{
    match v {
        Ok(items) => (
            succ_code,
            Json(ListResponse::<T::Serial> {
                success: true,
                response: Some(
                    items
                        .into_iter()
                        .map(|i| i.to_serial())
                        .collect::<Vec<T::Serial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            err_code,
            Json(ListResponse::<T::Serial> {
                success: false,
                response: None,
                error: Some(err),
            }),
        ),
    }
}

pub fn get_one_response<T>(
    r: Result<T, String>,
    succ_code: StatusCode,
    err_code: StatusCode,
) -> impl IntoResponse
where
    T: IntoSerial,
    T::Serial: Serialize,
{
    match r {
        Ok(item) => (
            succ_code,
            Json(Response::<T::Serial> {
                success: true,
                response: Some(item.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            err_code,
            Json(Response::<T::Serial> {
                success: false,
                response: None,
                error: Some(err),
            }),
        ),
    }
}
