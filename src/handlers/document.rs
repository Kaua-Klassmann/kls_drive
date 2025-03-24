use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use entity::document;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{fs::File, io::AsyncWriteExt};
use validator::Validate;

use crate::{jwt::JwtClaims, state::AppState};

pub async fn upload_document(
    State(state): State<AppState>,
    token: JwtClaims,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db = state.db_conn;

    let mut content_type: Option<String> = None;
    let mut name: Option<String> = None;
    let mut document_bytes: Vec<u8> = Vec::new();

    while let Ok(Some(mut field)) = multipart.next_field().await {
        if let Some(field_name) = field.name() {
            if field_name == "document" {
                if field.file_name().is_some() {
                    content_type = Some(field.content_type().unwrap().to_string());
                    name = Some(field.file_name().unwrap().to_string());

                    while let Ok(Some(chunk)) = field.chunk().await {
                        document_bytes.extend_from_slice(&chunk);
                    }
                }
            }
        }
    }

    if content_type.is_none() || name.is_none() || document_bytes.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid file"
            })),
        );
    }

    let document_exists = document::Entity::find()
        .filter(
            Condition::all()
                .add(document::Column::Name.eq(name.clone().unwrap()))
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
        name: Set(name.clone().unwrap()),
        r#type: Set(content_type.unwrap()),
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

    let mut file = File::create(format!(
        "./uploads/documents/{}/{}",
        token.user_id,
        name.unwrap()
    ))
    .await
    .unwrap();

    file.write(&document_bytes).await.unwrap();

    (StatusCode::OK, Json(json!({})))
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
