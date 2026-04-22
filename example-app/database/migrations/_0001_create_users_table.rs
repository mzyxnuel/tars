use async_trait::async_trait;
use tars_orm::{Migration, Schema};

pub struct CreateUsersTable;

#[async_trait]
impl Migration for CreateUsersTable {
    fn name(&self) -> &'static str {
        "0001_create_users_table"
    }

    async fn up(&self) -> Result<(), sqlx::Error> {
        Schema::create("users")
            .id()
            .string("name")
            .string("email")
            .timestamps()
            .execute()
            .await
    }

    async fn down(&self) -> Result<(), sqlx::Error> {
        Schema::drop("users").await
    }
}
