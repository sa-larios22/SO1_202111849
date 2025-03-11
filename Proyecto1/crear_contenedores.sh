#!/bin/bash

# Lista de opciones de contenedores con diferentes consumos
opciones=("RAM" "CPU" "IO" "DISK")

# Generar 10 contenedores aleatorios
for i in {1..10}; do
    # Seleccionar un tipo de consumo aleatorio
    tipo=${opciones[$((RANDOM % ${#opciones[@]}))]}

    # Generar un nombre único con la fecha y /dev/urandom y el tipo de consumo
    nombre="stress_$(date +%s)_$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 6)_$tipo"

    # Ejecutar el contenedor con el tipo de consumo seleccionado
    # -d hace que se ejecute en segundo plano
    # --rm elimina el contenedor al finalizar
    # --name asigna un nombre al contenedor
    # containerstack/alpine-stress es la imagen de Docker que contiene la herramienta stress
    # stress es la herramienta que genera el consumo
    # --vm 1 --vm-bytes 128M genera consumo de RAM
    # --cpu 1 genera consumo de CPU
    # --io 1 genera consumo de IO
    # --hdd 1 --hdd-bytes 256M genera consumo de disco
    # --timeout 60s finaliza el consumo a los 60 segundos
    case $tipo in
        "RAM")
            docker run -d --rm --name $nombre containerstack/alpine-stress stress --vm 1 --vm-bytes 128M --timeout 30s
            ;;
        "CPU")
            docker run -d --rm --name $nombre containerstack/alpine-stress stress --cpu 1 --timeout 30s
            ;;
        "IO")
            docker run -d --rm --name $nombre containerstack/alpine-stress stress --io 1 --timeout 30s
            ;;
        "DISK")
            docker run -d --rm --name $nombre containerstack/alpine-stress stress --hdd 1 --hdd-bytes 256M --timeout 30s
            ;;
    esac

    echo "Contenedor $nombre de tipo $tipo iniciado."
done

# La primera vez que lo ejecuté me asusté XD