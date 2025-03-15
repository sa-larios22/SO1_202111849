# Manual Técnico

## Generación de contenedores
El proyecto utiliza el script de bash `crear_contenedores.sh` que genera 10 contenedores de consumo aleatorio en Docker

Indica que el archivo debe ser ejecutado con bash
```bash
#!/bin/bash
```

Lista de opciones de contenedores con diferentes consumos
```bash
opciones=("RAM" "CPU" "IO" "DISK")
```

Genera 10 contenedores aleatorios de consumo aleatorio
```bash
for i in {1..10}; do
    tipo=${opciones[$((RANDOM % ${#opciones[@]}))]}
```

Nombre de la imagen
```bash
imagen="alpine_stress"
```

Genera un nombre único con la fecha y /dev/urandom y el tipo de consumo
```bash
nombre="stress_$(date +%s)_$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 6)_$tipo"
```

Ejecuta el contenedor con el tipo de consumo seleccionado
-d hace que se ejecute en segundo plano  
--rm elimina el contenedor al finalizar  
--name asigna un nombre al contenedor  
$nombre es el nombre del contenedor   
$imagen es el nombre de la imagen  

stress es la herramienta que genera el consumo  
--vm 1 --vm-bytes 128M genera consumo de RAM  
--cpu 1 genera consumo de 1 CPU  
--io 1 genera consumo de IO  
--hdd 1 --hdd-bytes 64M genera consumo de disco  
--timeout 60s finaliza el consumo a los 60 segundos  

```BASH
case $tipo in
    "RAM")
        docker run -d --rm --name $nombre $imagen stress --vm 1 --vm-bytes 128M --timeout 30s
        ;;
    "CPU")
        docker run -d --rm --name $nombre $imagen stress --cpu 1 --timeout 30s
        ;;
    "IO")
        docker run -d --rm --name $nombre $imagen stress --io 1 --timeout 30s
        ;;
    "DISK")
        docker run -d --rm --name $nombre $imagen stress --hdd 1 --hdd-bytes 16M --timeout 30s
        ;;
esac

echo "Contenedor $nombre de tipo $tipo iniciado."
```


## Cargar un Módulo de Kernel

Teniendo el código en C del módulo y un `Makefile` en el mismo directorio, se siguen los siguientes pasos en la terminal:

1. Realizar una instalación correcta del módulo
```bash
make clean          # Limpia compilaciones previas
make                # Compila el módulo
```

2. Verificar que `modulo.ko` se haya generado correctamente:
```bash
ls -lh modulo.ko
```

3. Cargar el módulo en el Kernel
```bash
sudo insmod modulo.ko
```

En caso de encontrar un error similar a:

```
sergio@sergio-ASUS-TUF:~/Descargas/SO1_202111849/Proyecto1/module$ sudo insmod module.ko
insmod: ERROR: could not insert module module.ko: Key was rejected by service
```

Se debe desactivar el SecureBoot de la UEFI/BIOS del dispositivo.

4. Verificar que el módulo se haya cargado correctamente
```bash
lsmod | grep modulo
```

Adicionalmente se pueden ver los logs de la terminal con
```bash
sudo dmesg | tail -20           # Muestra los últimos 20 mensajes de los logs
```

Y se puede ver el contenido del archivo `/proc/sysinfo_202111849` con el comando
```bash
cat /proc/sysinfo_202111849 | jq .
```

## Descargar un módulo de Kernel
Si se quiere eliminar el módulo del Kernel, se ejecuta:

```bash
sudo rmmod modulo
```

Se puede verificar que el módulo ya no está cargado con:
```bash
lsmod | grep modulo
```

Se pueden eliminar los archivos generados por la compilación del módulo de Kernel con:
```bash
make clean
```