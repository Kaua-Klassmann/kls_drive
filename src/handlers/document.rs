use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use entity::document;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, EntityTrait, FromQueryResult, IntoActiveModel,
    QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{fs, io::AsyncWriteExt};
use validator::Validate;

use crate::{jwt::JwtClaims, state::AppState};

pub async fn upload_document(
    State(state): State<AppState>,
    token: JwtClaims,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db = state.db_conn;

    while let Ok(Some(mut field)) = multipart.next_field().await {
        if field.name().is_none() || field.name().unwrap() != "document" {
            continue;
        }

        if field.file_name().is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid file"
                })),
            );
        }

        let content_type = field.content_type().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();

        let document_exists = document::Entity::find()
            .filter(
                Condition::all()
                    .add(document::Column::Name.eq(name.clone()))
                    .add(document::Column::IdUser.eq(token.user_id)),
            )
            .one(db)
            .await;

        if document_exists.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to upload document"
                })),
            );
        }

        if document_exists.unwrap().is_some() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "File already exists"
                })),
            );
        }

        let document = document::ActiveModel {
            id_user: Set(token.user_id),
            name: Set(name.clone()),
            r#type: Set(content_type),
            ..Default::default()
        };

        let document_result = document::Entity::insert(document).exec(db).await;

        if document_result.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to upload document"
                })),
            );
        }

        let mut file = fs::File::create(format!("./uploads/documents/{}/{}", token.user_id, name))
            .await
            .unwrap();

        while let Ok(Some(chunk)) = field.chunk().await {
            let mut offset = 0;
            while offset < chunk.len() {
                let writed = file.write(&chunk[offset..]).await;

                if writed.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": "Failed to save document"
                        })),
                    );
                }

                offset += writed.unwrap();
            }
        }

        return (StatusCode::OK, Json(json!({})));
    }

    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "Invalid schema"
        })),
    )
}

#[derive(Deserialize, Validate)]
pub struct ViewAllDocumentsPerPagePayload {
    #[validate(range(min = 1))]
    page: u64,
    #[validate(range(min = 10, max = 50))]
    limit: u64,
}

#[derive(FromQueryResult, Serialize)]
struct DocumentResponse {
    id: u32,
    name: String,
    r#type: String,
    created_at: NaiveDate,
}

pub async fn view_all_documents_per_page(
    State(state): State<AppState>,
    token: JwtClaims,
    Json(payload): Json<ViewAllDocumentsPerPagePayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() || !vec![10, 25, 50].contains(&payload.limit) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Invalid payload"
            })),
        );
    }

    let db = state.db_conn;

    let documents_result = document::Entity::find()
        .select_only()
        .columns([
            document::Column::Id,
            document::Column::Name,
            document::Column::Type,
            document::Column::CreatedAt,
        ])
        .filter(document::Column::IdUser.eq(token.user_id))
        .limit(Some(payload.limit))
        .offset(payload.limit * (payload.page - 1))
        .into_model::<DocumentResponse>()
        .all(db)
        .await;

    if documents_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to find documents"
            })),
        );
    }

    return (
        StatusCode::OK,
        Json(json!({"documents": documents_result.unwrap()})),
    );
}

pub async fn delete(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let db = state.db_conn;

    let document_result = document::Entity::find()
        .filter(document::Column::Id.eq(id))
        .one(db)
        .await;

    if document_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to find document"
            })),
        );
    }

    let document_option = document_result.unwrap();

    if document_option.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Document not exists"
            })),
        );
    }

    let document = document_option.unwrap();

    if document.id_user != token.user_id {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Document is of another user"
            })),
        );
    }

    let document_name = document.name.clone();

    let delete_result = document::Entity::delete(document.into_active_model())
        .exec(db)
        .await;

    if delete_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to delete document"
            })),
        );
    }

    let _ = fs::remove_file(format!(
        "./uploads/documents/{}/{}",
        token.user_id, document_name
    ))
    .await;

    (StatusCode::OK, Json(json!({})))
}
