use crate::core::FaceEngine;
use crate::store::FaceStorage;
use crate::core::types::{IdentificationResult, EngineError, StorageError};
use image::load_from_memory;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Image error: {0}")]
    Image(String),
}

pub struct FaceService {
    engine: Arc<dyn FaceEngine>,
    storage: Arc<dyn FaceStorage>,
}

impl FaceService {
    pub fn new(engine: Arc<dyn FaceEngine>, storage: Arc<dyn FaceStorage>) -> Self {
        Self { engine, storage }
    }

    pub async fn enroll_user(&self, user_id: &str, image_bytes: &[u8]) -> Result<(), ServiceError> {
        let img = load_from_memory(image_bytes)
            .map_err(|e| ServiceError::Image(e.to_string()))?;
        
        let detections = self.engine.detect_faces(&img)?;
        let primary_face = detections.first()
            .ok_or_else(|| ServiceError::Image("No face detected".to_string()))?;
        
        let embedding = self.engine.extract_embedding(&img, primary_face)?;
        
        self.storage.save_face(user_id, embedding).await?;
        
        Ok(())
    }

    pub async fn identify_user(&self, image_bytes: &[u8]) -> Result<Option<IdentificationResult>, ServiceError> {
        let img = load_from_memory(image_bytes)
            .map_err(|e| ServiceError::Image(e.to_string()))?;
        
        let detections = self.engine.detect_faces(&img)?;
        let primary_face = detections.first()
            .ok_or_else(|| ServiceError::Image("No face detected".to_string()))?;
        
        let embedding = self.engine.extract_embedding(&img, primary_face)?;
        
        let results = self.storage.search_face(embedding, 1).await?;
        
        Ok(results.into_iter().next())
    }
}
