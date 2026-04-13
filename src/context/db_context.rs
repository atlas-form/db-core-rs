use std::{future::Future, pin::Pin, sync::Arc};

use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::{DbTransaction, error};

/// tables 对外暴露的数据库上下文
///
/// 约定：
/// - 外部 crate 只能“拿着它用”
/// - 不能依赖 SeaORM
#[derive(Clone)]
pub struct DbContext {
    inner: Arc<DatabaseConnection>,
}

impl DbContext {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { inner: db }
    }

    pub(crate) fn inner(&self) -> &DatabaseConnection {
        &self.inner
    }

    /// Begin a database transaction and return wrapped transaction handle.
    pub async fn begin(&self) -> error::Result<DbTransaction> {
        let tx = self.inner.begin().await?;
        Ok(DbTransaction::new(tx))
    }

    /// Execute operations in one transaction.
    ///
    /// - callback returns `Ok(T)`: commit
    /// - callback returns `Err(Error)`: rollback
    pub async fn transaction<F, T>(&self, callback: F) -> error::Result<T>
    where
        F: for<'c> FnOnce(
                &'c DbTransaction,
            )
                -> Pin<Box<dyn Future<Output = error::Result<T>> + Send + 'c>>
            + Send,
        T: Send,
    {
        let tx = self.begin().await?;
        match callback(&tx).await {
            Ok(value) => {
                tx.commit().await?;
                Ok(value)
            }
            Err(err) => {
                if let Err(rb_err) = tx.rollback().await {
                    return Err(error::Error::Transaction(format!(
                        "{err}; rollback failed: {rb_err}"
                    )));
                }
                Err(err)
            }
        }
    }
}
