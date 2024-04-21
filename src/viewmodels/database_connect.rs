use sea_orm::{DatabaseConnection,Database, DbErr};
// use sea_orm::TryGetError::DbErr;
pub async fn connect_to_database() -> Result<DatabaseConnection,DbErr> {
    let db_string_connection = Database::connect("Data Source=(localdb)\\MSSQLLocalDB;Initial Catalog=ShopDB;Integrated Security=True;").await?;

    // let mut opt = ConnectionOptions::new("mysql://username:password@host/database".to_owned());
    
    Ok(db_string_connection)
}