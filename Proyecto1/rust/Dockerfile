# Usa la imagen oficial de Rust
FROM rust:latest

# Crea y establece el directorio de trabajo
WORKDIR /usr/src/app

# Copia los archivos del proyecto al contenedor
COPY . .

# Instalación de Docker dentro del contenedor (para poder usarlo)
RUN apt-get update && apt-get install -y docker.io

# Comando para ejecutar el programa de Rust
CMD ["cargo", "run"]
