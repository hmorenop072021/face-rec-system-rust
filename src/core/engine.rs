use tract_onnx::prelude::*;
use image::{DynamicImage};
use crate::core::types::{FaceDetection, FaceEmbedding, EngineError};
use crate::core::FaceEngine;
use std::sync::Arc;

pub struct TractEngine {
    model: Arc<SimplePlan<TypedFact, Box<dyn TypedOp>, TypedModel>>,
}

impl TractEngine {
    pub fn new(model_path: &str) -> Result<Self, EngineError> {
        let model = onnx()
            .model_for_path(model_path)
            .map_err(|e: anyhow::Error| EngineError::LoadError(e.to_string()))?
            .with_input_fact(0, f32::fact(&[1, 3, 112, 112]).into())
            .map_err(|e: anyhow::Error| EngineError::LoadError(e.to_string()))?
            .into_optimized()
            .map_err(|e: anyhow::Error| EngineError::LoadError(e.to_string()))?
            .into_runnable()
            .map_err(|e: anyhow::Error| EngineError::LoadError(e.to_string()))?;

        Ok(Self {
            model: Arc::new(model),
        })
    }
}

impl FaceEngine for TractEngine {
    fn detect_faces(&self, _image: &DynamicImage) -> Result<Vec<FaceDetection>, EngineError> {
        Ok(vec![FaceDetection {
            box_area: [0.0, 0.0, 112.0, 112.0],
            confidence: 1.0,
        }])
    }

    fn extract_embedding(&self, image: &DynamicImage, _detection: &FaceDetection) -> Result<FaceEmbedding, EngineError> {
        let resized = image.resize_exact(112, 112, image::imageops::FilterType::Lanczos3);
        let rgb = resized.to_rgb8();
        
        let mut tensor = Tensor::zero::<f32>(&[1, 3, 112, 112]).map_err(|e: anyhow::Error| EngineError::ImageError(e.to_string()))?;
        let slice = tensor.as_slice_mut::<f32>().map_err(|e: anyhow::Error| EngineError::ImageError(e.to_string()))?;
        
        for (y, row) in rgb.enumerate_rows() {
            for (x, _y, pixel) in row {
                slice[0 * 112 * 112 + y as usize * 112 + x as usize] = (pixel[0] as f32 - 127.5) / 128.0;
                slice[1 * 112 * 112 + y as usize * 112 + x as usize] = (pixel[1] as f32 - 127.5) / 128.0;
                slice[2 * 112 * 112 + y as usize * 112 + x as usize] = (pixel[2] as f32 - 127.5) / 128.0;
            }
        }

        let result = self.model.run(tvec!(tensor.into())).map_err(|e: anyhow::Error| EngineError::InferenceError(e.to_string()))?;
        let embedding: Vec<f32> = result[0].as_slice::<f32>().map_err(|e: anyhow::Error| EngineError::InferenceError(e.to_string()))?.to_vec();

        Ok(FaceEmbedding(embedding))
    }
}
