//! The Postgres `LedgerStore`, over the spend log repository.

use litellm_db::Error;
use litellm_repositories::{SpendLogRepository, SpendLogWrite};

use crate::LedgerStore;

/// Commits statements to Postgres through the spend log repository, which reports each
/// statement and applies the deadlines.
pub struct PgLedgerStore {
    repository: SpendLogRepository,
}

impl PgLedgerStore {
    pub fn new(repository: SpendLogRepository) -> Self {
        Self { repository }
    }
}

impl LedgerStore for PgLedgerStore {
    async fn insert(&self, writes: &[SpendLogWrite]) -> Result<usize, Error> {
        self.repository.insert_with_deliveries(writes).await
    }
}
