# Imagen oficial de Rust como base para la construcción
FROM rust:1.86 as builder

# Establece el directorio de trabajo dentro del contenedor
WORKDIR /app

# Copia el proyecto al contenedor
COPY . .

# Compila el proyecto en modo release
RUN cargo build --release

# Imagen base para el contenedor final
FROM debian:bookworm-slim

# Instala dependencias
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copia solo el binario ya compilado desde la primera imagen al contenedor final.
COPY --from=builder /app/target/release/rust-api /usr/local/bin/rust-api

# Expone el puerto 8080 para el contenedor
EXPOSE 8080

# Define el comando por defecto que se ejecutará al iniciar el contenedor.
CMD ["rust-api"]
