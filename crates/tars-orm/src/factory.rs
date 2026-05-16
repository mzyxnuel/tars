use async_trait::async_trait;

use crate::model::Model;

/// Laravel-style model factory. Implement `definition` to return the
/// default attributes, then call `create_many` to persist a batch.
#[async_trait]
pub trait Factory: Send + Sync {
    type M: Model;

    /// Default attribute set for a single record.
    fn definition(&self) -> serde_json::Value;

    /// Generate `count` attribute maps. Override to vary values.
    fn make(&self, count: usize) -> Vec<serde_json::Value> {
        (0..count).map(|_| self.definition()).collect()
    }

    /// Persist `count` records via `Model::create`. Returns the
    /// persisted models (each one freshly fetched from the database).
    async fn create_many(&self, count: usize) -> Result<Vec<Self::M>, sqlx::Error> {
        let mut out = Vec::with_capacity(count);
        for attrs in self.make(count) {
            out.push(Self::M::create(attrs).await?);
        }
        Ok(out)
    }
}
