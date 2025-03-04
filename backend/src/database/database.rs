use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::database::get_db_config;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_connections() -> DatabaseConnection {
    DB.get_or_init(|| async {
        let db = Database::connect(get_db_config())
            .await
            .expect("Failed to connect to the database");

        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to migrate the database");

        db
    })
    .await
    .clone()
}
