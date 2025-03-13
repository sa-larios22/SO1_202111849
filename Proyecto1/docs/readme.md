# Manual Técnico

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