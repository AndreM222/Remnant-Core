use sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: &str) -> anyhow::Result<DatabaseConnection> {
    let database = Database::connect(url).await?;
    Ok(database)
}
