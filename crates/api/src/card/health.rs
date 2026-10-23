use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use utoipa::{OpenApi, ToSchema};
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::models::{ApiErrorCode, ErrorResponse};

/// Card health response when enabled.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardHealthResponse {
    /// Whether the card feature is enabled.
    pub enabled: bool,
    /// Issuer (partner) stellar address or "partner_unconfigured".
    pub issuer: String,
}

/// Error response for 404 when card is disabled.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardNotEnabledResponse {
    pub error: String,
}

/// GET /api/v1/card/health
#[utoipa::path(
    get,
    path = "/api/v1/card/health",
    tag = "card",
    responses(
        (status = 200, description = "Card health status", body = CardHealthResponse),
        (status = 404, description = "Card feature not enabled", body = CardNotEnabledResponse),
    )
)]
pub async fn card_health(
    State(state): State<crate::state::AppStateArc>,
) -> Result<Json<crate::models::ApiResponse<CardHealthResponse>>, (StatusCode, Json<crate::models::ApiResponse<CardNotEnabledResponse>>)> {
    let card_config = &state.card_config;
    
    if !card_config.enabled {
        return Err((
            StatusCode::NOT_FOUND,
            Json(crate::models::ApiResponse::with_version(
                1,
                CardNotEnabledResponse {
                    error: "Card feature not enabled".to_string(),
                },
                "",
            )),
        ));
    }

    Ok(Json(crate::models::ApiResponse::with_version(
        1,
        CardHealthResponse {
            enabled: true,
            issuer: card_config.issuer().to_string(),
        },
        "",
    )))
}