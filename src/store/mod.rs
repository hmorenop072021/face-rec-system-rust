pub mod qdrant;

use crate::core::types::{FaceEmbedding, IdentificationResult, StorageError};
use async_trait::async_trait;

#[async_trait]
pub trait FaceStorage: Send + Sync {
    async fn save_face(&self, user_id: &str, embedding: FaceEmbedding) -> Result<(), StorageError>;
    async fn search_face(&self, embedding: FaceEmbedding, limit: usize) -> Result<Vec<IdentificationResult>, StorageError>;
}
