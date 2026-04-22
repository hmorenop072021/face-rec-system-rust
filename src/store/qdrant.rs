use qdrant_client::client::{QdrantClient, Payload};
use qdrant_client::qdrant::{
    PointStruct, SearchPoints, CreateCollection, Distance,
};
use crate::core::types::{FaceEmbedding, IdentificationResult, StorageError};
use crate::store::FaceStorage;
use async_trait::async_trait;
use std::sync::Arc;

pub struct QdrantStorage {
    client: Arc<QdrantClient>,
    collection_name: String,
}

impl QdrantStorage {
    pub async fn new(url: &str, collection_name: &str) -> Result<Self, StorageError> {
        let client = QdrantClient::from_url(url)
            .build()
            .map_err(|e: anyhow::Error| StorageError::ConnectionError(e.to_string()))?;
        
        let storage = Self {
            client: Arc::new(client),
            collection_name: collection_name.to_string(),
        };

        let has_col: bool = storage.client.has_collection(storage.collection_name.clone()).await.map_err(|e: anyhow::Error| StorageError::ConnectionError(e.to_string()))?;
        
        if !has_col {
            storage.client
                .create_collection(&CreateCollection {
                    collection_name: storage.collection_name.clone(),
                    vectors_config: Some(qdrant_client::qdrant::VectorsConfig {
                        config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                            qdrant_client::qdrant::VectorParams {
                                size: 512,
                                distance: Distance::Cosine as i32,
                                ..Default::default()
                            }
                        )),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e: anyhow::Error| StorageError::ConnectionError(e.to_string()))?;
        }

        Ok(storage)
    }
}

#[async_trait]
impl FaceStorage for QdrantStorage {
    async fn save_face(&self, user_id: &str, embedding: FaceEmbedding) -> Result<(), StorageError> {
        let mut payload = Payload::new();
        payload.insert("user_id", user_id.to_string());
        
        let point = PointStruct::new(
            uuid::Uuid::new_v4().to_string(), 
            embedding.0,
            payload
        );

        let _: qdrant_client::qdrant::PointsOperationResponse = self.client
            .upsert_points(self.collection_name.clone(), None, vec![point], None)
            .await
            .map_err(|e: anyhow::Error| StorageError::SaveError(e.to_string()))?;

        Ok(())
    }

    async fn search_face(&self, embedding: FaceEmbedding, limit: usize) -> Result<Vec<IdentificationResult>, StorageError> {
        let request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: embedding.0,
            limit: limit as u64,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let search_result: qdrant_client::qdrant::SearchResponse = self.client
            .search_points(&request)
            .await
            .map_err(|e: anyhow::Error| StorageError::SearchError(e.to_string()))?;

        let results = search_result.result
            .into_iter()
            .map(|point| {
                let user_id = point.payload.get("user_id")
                    .and_then(|v| v.as_str())
                    .map_or("unknown", |v| v)
                    .to_string();
                IdentificationResult {
                    user_id,
                    score: point.score,
                }
            })
            .collect();

        Ok(results)
    }
}
