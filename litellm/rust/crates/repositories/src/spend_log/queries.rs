//! The spend log ledger's statements on `LiteLLM_SpendLogs` and `LiteLLM_SpendLogDelivery`,
//! mirroring the Python ledger writer (`SpendLogsRepository._insert_ledger`).

use serde_json::Value;
use sqlx::PgExecutor;

use litellm_db::Error;

/// Inserts the request logs in `records` (a JSON array of `LiteLLM_SpendLogs` rows), skipping
/// any `request_id` already stored, and returns the ids it inserted.
pub(super) async fn insert_spend_log_rows(
    executor: impl PgExecutor<'_>,
    records: Value,
) -> Result<Vec<String>, Error> {
    Ok(
        sqlx::query_file_scalar!("src/spend_log/queries/insert_spend_log_rows.sql", records)
            .fetch_all(executor)
            .await?,
    )
}

/// Inserts the deliveries in `payloads` (a JSON array of delivery rows), skipping any
/// `delivery_id` already stored, and returns the ids it accepted.
pub(super) async fn insert_spend_log_deliveries(
    executor: impl PgExecutor<'_>,
    payloads: Value,
) -> Result<Vec<String>, Error> {
    Ok(sqlx::query_file_scalar!(
        "src/spend_log/queries/insert_spend_log_deliveries.sql",
        payloads
    )
    .fetch_all(executor)
    .await?)
}

/// Which of the settle-once `request_ids` a pre-ledger cost row already charged.
pub(super) async fn spend_log_jobs_charged_by_legacy(
    executor: impl PgExecutor<'_>,
    request_ids: &[String],
) -> Result<Vec<String>, Error> {
    Ok(sqlx::query_file_scalar!(
        "src/spend_log/queries/select_spend_log_jobs_charged_by_legacy.sql",
        request_ids
    )
    .fetch_all(executor)
    .await?)
}

/// Removes the deliveries in `delivery_ids`.
pub(super) async fn delete_spend_log_deliveries(
    executor: impl PgExecutor<'_>,
    delivery_ids: &[String],
) -> Result<(), Error> {
    sqlx::query_file!(
        "src/spend_log/queries/delete_spend_log_deliveries.sql",
        delivery_ids
    )
    .execute(executor)
    .await?;
    Ok(())
}
