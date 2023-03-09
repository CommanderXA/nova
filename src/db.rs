use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn run() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    Ok(db)
}
