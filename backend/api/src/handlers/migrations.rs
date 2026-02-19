use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use shared::models::{
    CreateMigrationRequest, Migration, MigrationStatus, PaginatedResponse,
    UpdateMigrationStatusRequest,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::state::AppState;

/// Create a new migration
pub async fn create_migration(
    State(state): State<AppState>,
    Json(payload): Json<CreateMigrationRequest>,
) -> Result<Json<Migration>, AppError> {
    let migration = sqlx::query_as!(
        Migration,
        r#"
        INSERT INTO migrations (contract_id, wasm_hash, status)
        VALUES ($1, $2, 'pending')
        RETURNING id, contract_id, status as "status: MigrationStatus", wasm_hash, log_output, created_at, updated_at
        "#,
        payload.contract_id,
        payload.wasm_hash
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(migration))
}

/// Update a migration status
pub async fn update_migration(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMigrationStatusRequest>,
) -> Result<Json<Migration>, AppError> {
    let migration = sqlx::query_as!(
        Migration,
        r#"
        UPDATE migrations
        SET status = $1, log_output = COALESCE($2, log_output)
        WHERE id = $3
        RETURNING id, contract_id, status as "status: MigrationStatus", wasm_hash, log_output, created_at, updated_at
        "#,
        payload.status as MigrationStatus,
        payload.log_output,
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(migration))
}

/// Get all migrations
pub async fn get_migrations(
    State(state): State<AppState>,
) -> Result<Json<PaginatedResponse<Migration>>, AppError> {
    // For simplicity, we'll just return the last 50 migrations
    let migrations = sqlx::query_as!(
        Migration,
        r#"
        SELECT id, contract_id, status as "status: MigrationStatus", wasm_hash, log_output, created_at, updated_at
        FROM migrations
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let total = migrations.len() as i64; // In a real app we'd do a count query
    let response = PaginatedResponse::new(migrations, total, 1, 50);

    Ok(Json(response))
}

/// Get a specific migration
pub async fn get_migration(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Migration>, AppError> {
    let migration = sqlx::query_as!(
        Migration,
        r#"
        SELECT id, contract_id, status as "status: MigrationStatus", wasm_hash, log_output, created_at, updated_at
        FROM migrations
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound("Migration not found".to_string()))?;

    Ok(Json(migration))
}
