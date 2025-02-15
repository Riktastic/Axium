use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct SuccessResponse {
    message: String,
}

#[derive(ToSchema)]
pub struct ErrorResponse {
    error: String,
}
