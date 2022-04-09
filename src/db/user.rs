// LIBS
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::user::User;
use crate::config::error::MyError;


pub async fn create_user(client: &Client, user_model: User) -> Result<User, MyError> {
    // sql instruction
    let _statement = include_str!("sql\\user_create.sql");
    let _statement = _statement.replace("$table_fields", &User::sql_table_fields());
    let statement = client.prepare(&_statement).await.unwrap();

    // process
    client
        .query(
            &statement,
            &[
                &user_model.username,
                &user_model.password,
                &user_model.fullname,
                &user_model.email,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}