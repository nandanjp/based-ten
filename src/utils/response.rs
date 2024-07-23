use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

pub fn get_list_response<T>(
    v: Result<Vec<T>, String>,
    succ_code: StatusCode,
    err_code: StatusCode,
) -> impl IntoResponse
where
    T: Serialize,
{
    match v {
        Ok(items) => (
            succ_code,
            Json(serde_json::json!({
                "success": true,
                "response": items,
                "error": None::<String>
            })),
        ),
        Err(err) => (
            err_code,
            Json(serde_json::json!({
                "success": true,
                "response": None::<Vec<T>>,
                "error": Some(err)
            })),
        ),
    }
}

pub fn get_one_response<T>(
    r: Result<T, String>,
    succ_code: StatusCode,
    err_code: StatusCode,
) -> impl IntoResponse
where
    T: Serialize,
{
    match r {
        Ok(item) => (
            succ_code,
            Json(serde_json::json!({
                "success": true,
                "response": Some(item),
                "error": None::<String>
            })),
        ),
        Err(err) => (
            err_code,
            Json(serde_json::json!({
                "success": true,
                "response": None::<T>,
                "error": Some(err)
            })),
        ),
    }
}
