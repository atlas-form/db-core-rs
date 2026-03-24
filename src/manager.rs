use std::{collections::HashMap, sync::Arc, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{debug, info};

use crate::{
    DbContext,
    config::DatabaseConfig,
    error::{Error, Result},
};

/// Multi-database connection manager
pub struct DatabaseManager {
    connections: HashMap<String, DbContext>,
}

impl DatabaseManager {
    /// Create a new DatabaseManager with the given configurations
    pub async fn new(configs: Vec<DatabaseConfig>) -> Result<Self> {
        if configs.is_empty() {
            return Err(Error::Custom(
                "At least one database configuration is required".to_owned(),
            ));
        }

        let mut connections = HashMap::new();

        for config in configs {
            info!("Connecting to database: {}", config.name);

            let mut opt = ConnectOptions::new(&config.url);
            opt.max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect_timeout(Duration::from_secs(config.connect_timeout))
                .idle_timeout(Duration::from_secs(config.idle_timeout))
                .sqlx_logging(config.sql_logging);

            let db = Database::connect(opt).await.map_err(|e| {
                Error::Custom(format!("Connection failed for {}: {}", config.name, e))
            })?;

            let ctx = conn_to_context(db);

            debug!("Successfully connected to database: {}", config.name);
            connections.insert(config.name.clone(), ctx);
        }

        Ok(Self { connections })
    }

    /// Get a database connection by name
    pub fn get(&self, name: &str) -> Result<DbContext> {
        self.connections
            .get(name)
            .cloned()
            .ok_or_else(|| Error::Custom(format!("Database not found: {}", name)))
    }

    /// Get the default database connection (first one added)
    pub fn default(&self) -> DbContext {
        self.connections
            .values()
            .next()
            .cloned()
            .expect("DatabaseManager has no database connections")
    }

    /// Get all database connection names
    pub fn list_databases(&self) -> Vec<&String> {
        self.connections.keys().collect()
    }

    /// Check if a database connection exists
    pub fn has_database(&self, name: &str) -> bool {
        self.connections.contains_key(name)
    }

    /// Get the number of database connections
    pub fn count(&self) -> usize {
        self.connections.len()
    }
}

fn conn_to_context(conn: DatabaseConnection) -> DbContext {
    DbContext::new(Arc::new(conn))
}
