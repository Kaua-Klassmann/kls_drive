use axum::{
    Json,
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::document;
use sea_orm::{ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde_json::json;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{jwt::JwtClaims, state::AppState};

pub async fn upload_document(
    State(state): State<AppState>,
    token: JwtClaims,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db = state.db_conn;

    let mut content_type: Option<String> = None;
    let mut name: Option<String> = None;
    let mut document_bytes: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();

        if field_name == "document" {
            if field.file_name().is_some() {
                content_type = Some(field.content_type().unwrap().to_string());
                name = Some(field.file_name().unwrap().to_string());
                document_bytes = Some(field.bytes().await.unwrap());
            }
        }
    }

    if content_type.is_none() || name.is_none() || document_bytes.is_none() {
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

    file.write(&document_bytes.unwrap()).await.unwrap();

    (StatusCode::OK, Json(json!({})))
}
