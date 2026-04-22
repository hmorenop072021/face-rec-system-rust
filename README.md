# Face Recognition System 1:N (Rust)

Sistema industrial de reconocimiento facial de alto rendimiento diseñado bajo la metodología **Spec-Driven Design (SDD)**.

## 🤖 Contexto para Agentes de IA
Este proyecto utiliza un enfoque de **contratos estrictos** para evitar alucinaciones en la generación de código.
- **Motor de IA:** `tract-onnx` (Rust puro) con modelos ArcFace (112x112, 512-dim).
- **Base de Datos:** `Qdrant` (Vector DB en Rust) con Distancia Coseno.
- **Orquestación:** `Axum` (Web) + `Tokio` (Async).
- **Contratos:** Ver `src/core/mod.rs` (`FaceEngine`) y `src/store/mod.rs` (`FaceStorage`).

## 🚀 Arquitectura
1. **Ingesta:** API REST con Axum manejando Multipart para imágenes.
2. **Inferencia:** Preprocesamiento de imagen (112x112, norm) -> Embedding de 512 floats.
3. **Búsqueda:** 1:N en Qdrant para identificación instantánea.

## 🛠️ Requisitos
- **Rust** 1.75+
- **Docker & Docker Compose**
- Modelo **`arcface.onnx`** en la carpeta `models/`.

## 📦 Instalación y Uso
1. Clonar el repositorio.
2. Descargar el modelo ArcFace y guardarlo en `models/arcface.onnx`.
3. Levantar los servicios:
   ```bash
   docker-compose up --build
   ```

## 📍 Endpoints
- `POST /enroll`: Registra un usuario (`user_id` y `image`).
- `POST /identify`: Identifica a un usuario a partir de una `image`.

---
*Proyecto generado y optimizado mediante Gemini CLI y Qwen 2.5 Coder.*
