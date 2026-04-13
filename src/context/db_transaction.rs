use sea_orm::{DatabaseTransaction as SeaDatabaseTransaction, TransactionTrait};

use crate::error::Result;

/// Wrapped database transaction handle.
///
/// This type hides SeaORM transaction details from external callers.
pub struct DbTransaction {
    inner: SeaDatabaseTransaction,
}

impl DbTransaction {
    pub(crate) fn new(inner: SeaDatabaseTransaction) -> Self {
        Self { inner }
    }

    /// Begin nested transaction (SAVEPOINT).
    pub async fn begin(&self) -> Result<DbTransaction> {
        let tx = self.inner.begin().await?;
        Ok(DbTransaction::new(tx))
    }

    /// Commit transaction.
    pub async fn commit(self) -> Result<()> {
        self.inner.commit().await?;
        Ok(())
    }

    /// Rollback transaction.
    pub async fn rollback(self) -> Result<()> {
        self.inner.rollback().await?;
        Ok(())
    }
}
