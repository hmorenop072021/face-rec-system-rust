mod core;
mod store;
mod api;

use std::sync::Arc;
use axum::{
    routing::post,
    Router,
};
use crate::core::engine::TractEngine;
use crate::store::qdrant::QdrantStorage;
use crate::core::service::FaceService;
use crate::api::handlers::{enroll_handler, identify_handler};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Iniciando Sistema de Reconocimiento Facial...");

    // 1. Inicializar Motor de Inferencia (ArcFace ONNX)
    // Nota: El usuario debe proveer el archivo 'arcface.onnx' en la carpeta models/
    let model_path = "models/arcface.onnx";
    let engine = Arc::new(TractEngine::new(model_path)?);

    // 2. Inicializar Almacén Vectorial (Qdrant)
    let qdrant_url = "http://localhost:6334";
    let storage = Arc::new(QdrantStorage::new(qdrant_url, "faces").await?);

    // 3. Crear el Servicio Orquestador
    let service = Arc::new(FaceService::new(engine, storage));

    // 4. Configurar Rutas de la API
    let app = Router::new()
        .route("/enroll", post(enroll_handler))
        .route("/identify", post(identify_handler))
        .with_state(service);

    // 5. Arrancar el Servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Servidor escuchando en http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
