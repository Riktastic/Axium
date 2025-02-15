use serde::Serialize;
use utoipa::ToSchema;

/// Represents the usage statistics for the last 24 hours.
#[derive(Debug, Serialize, ToSchema)]
pub struct UsageResponseLastDay {
    /// The number of requests made in the last 24 hours.
    #[serde(rename = "requests_last_24_hours")]
    pub count: i64
}

/// Represents the usage statistics for the last 7 days.
#[derive(Debug, Serialize, ToSchema)]
pub struct UsageResponseLastWeek {
    /// The number of requests made in the last 7 days.
    #[serde(rename = "requests_last_7_days")]
    pub count: i64
}
