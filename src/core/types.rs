use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceEmbedding(pub Vec<f32>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceDetection {
    pub box_area: [f32; 4], // [x, y, w, h]
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentificationResult {
    pub user_id: String,
    pub score: f32,
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Inference failed: {0}")]
    InferenceError(String),
    #[error("Model load failed: {0}")]
    LoadError(String),
    #[error("Image processing failed: {0}")]
    ImageError(String),
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection failed: {0}")]
    ConnectionError(String),
    #[error("Search failed: {0}")]
    SearchError(String),
    #[error("Save failed: {0}")]
    SaveError(String),
}
