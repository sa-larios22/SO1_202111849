# Imagen oficial de Rust como base para la construcción
FROM rust:1.86 as builder

# Establece el directorio de trabajo dentro del contenedor
WORKDIR /app

# Copia el proyecto al contenedor
COPY . .

# Instala dependencias
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Imagen base para el contenedor final
FROM ubuntu:22.04

# Copia solo el binario ya compilado desde la primera imagen al contenedor final.
COPY --from=builder /app/target/release/rust-api /usr/local/bin/rust-api

# Define el comando por defecto que se ejecutará al iniciar el contenedor.
CMD ["api"]
