# Builder Stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app
# Instalar dependencias de compilación para Rust
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copiar archivos de definición de dependencias
COPY Cargo.toml Cargo.lock ./

# Crear un dummy main.rs para cachear la compilación de dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/face_recognition_system*

# Copiar el código fuente real
COPY . .

# Compilar la aplicación real
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Instalar librerías de tiempo de ejecución (OpenSSL es necesaria para clientes de red)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copiar el binario desde el builder
COPY --from=builder /app/target/release/face-recognition-system /app/face-api

# Exponer el puerto de la API
EXPOSE 3000

# Comando para ejecutar la aplicación
CMD ["./face-api"]
