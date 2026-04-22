pub mod types;
pub mod engine;
pub mod service;

use image::DynamicImage;
pub use crate::core::types::{FaceDetection, FaceEmbedding, EngineError};

pub trait FaceEngine: Send + Sync {
    fn detect_faces(&self, image: &DynamicImage) -> Result<Vec<FaceDetection>, EngineError>;
    fn extract_embedding(&self, image: &DynamicImage, detection: &FaceDetection) -> Result<FaceEmbedding, EngineError>;
}
