use async_trait::async_trait;
use tars_orm::{Factory, Seeder};

use crate::database::factories::UserFactory;

pub struct UserSeeder;

#[async_trait]
impl Seeder for UserSeeder {
    fn name(&self) -> &'static str {
        "UserSeeder"
    }

    async fn run(&self) -> Result<(), sqlx::Error> {
        UserFactory.create_many(5).await?;
        Ok(())
    }
}
