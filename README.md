# Embebidos
# Proyecto Yocto + Rust + OpenCV
### Taller de Sistemas Embebidos — I Semestre 2026
**Instituto Tecnológico de Costa Rica | Escuela de Ingeniería Electrónica**  
Profesor: Dr. Ing. Johan Carvajal Godínez

---

## Descripción

Este repositorio contiene el entorno completo para generar una imagen Linux personalizada con **Yocto Project (Kirkstone)** que integra una aplicación de **detección de rostros** escrita en **Rust** usando la biblioteca **OpenCV**. La imagen está compilada para arquitectura `x86_64` y puede ejecutarse en QEMU o en VirtualBox.

---

## Requisitos previos

Antes de comenzar, asegúrese de contar con lo siguiente en su máquina host:

- Sistema operativo: **Linux (Ubuntu 22.04 recomendado)** o Windows/macOS con Docker Desktop
- **Docker Engine** y **Docker Compose** v2.x instalados
- Mínimo **8 GB de RAM** disponibles para el contenedor
- Mínimo **50 GB de espacio libre** en disco
- Conexión a Internet activa

---

## Estructura del repositorio

```
Embebidos/
├── vision_app/              # Aplicación Rust con OpenCV
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Cargo.lock
├── yocto-docker/            # Entorno Docker para Yocto
│   ├── Dockerfile
│   ├── docker-compose.yml
│   ├── setup-yocto.sh       # Script de configuración automática
│   ├── layers/
│   │   ├── meta-rust-opencv/  # Receta para la app Rust
│   │   └── meta-local/        # Fix para libxcb
│   └── conf/
│       ├── bblayers.conf.example
│       └── local.conf.example
└── README.md
```

---

## Configuración del entorno Docker

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/Embebidos.git
cd Embebidos
```

### 2. Crear el directorio de workspace

> ⚠️ Cree la carpeta antes de levantar Docker para evitar problemas de permisos:

```bash
mkdir -p yocto-docker/workspace
```

### 3. Construir y levantar el contenedor

```bash
cd yocto-docker
docker compose build
docker compose up -d
```

Esto crea el contenedor `yocto-rust-opencv` con todas las herramientas de compilación y configura dos volúmenes persistentes:

| Volumen local | Ruta en contenedor | Propósito |
|---|---|---|
| `./workspace` | `/workspace` | Builds de Yocto (persistencia) |
| `./layers` | `/workspace/layers` | Capas personalizadas |

> **Nota:** Las capas `meta-rust-opencv` y `meta-local` deben estar presentes en `yocto-docker/layers/` antes de ejecutar el script de configuración. Al clonar el repositorio ya estarán ahí.

### 4. Verificar que el contenedor está corriendo

```bash
docker ps
```

Debe aparecer `yocto-rust-opencv` con estado `Up`.

---

## Construcción de la imagen Yocto

### 5. Ingresar al contenedor

```bash
docker exec -it yocto-rust-opencv bash
```

### 6. Ejecutar el script de configuración

```bash
cd /workspace
./setup-yocto.sh
```

El script realiza automáticamente las siguientes acciones:

- Clona **Poky** (rama `kirkstone`), `meta-openembedded` y `meta-rust-bin`
- Copia las capas `meta-rust-opencv` y `meta-local` dentro de `poky/`
- Inicializa el entorno de construcción con `oe-init-build-env`
- Agrega todas las capas a `bblayers.conf`
- Configura `local.conf` con `MACHINE = "qemux86-64"` y optimizaciones de compilación

### 7. Cargar el entorno de Yocto

```bash
su - yocto
cd /workspace/poky
source oe-init-build-env
```

### 8. Compilar la imagen

```bash
bitbake vision-image
```

> ⚠️ La compilación puede tomar entre **1 y 4 horas** dependiendo del hardware del host. Las compilaciones posteriores son más rápidas gracias al sistema de caché (sstate).

Los artefactos generados se encuentran en:

```
/workspace/poky/build/tmp/deploy/images/qemux86-64/
```

Los archivos más relevantes son:

| Archivo | Uso |
|---|---|
| `vision-image-qemux86-64.wic.vmdk` | Imagen para VirtualBox |
| `vision-image-qemux86-64.rootfs.ext4` | Filesystem raw |
| `vision-image-qemux86-64.rootfs.tar.bz2` | Rootfs comprimido |

---

## Preparar el video de prueba

La receta instala el binario en `/usr/bin/vision_app`. El video `Prueba.mp4` debe copiarse manualmente al sistema.

**Desde el host**, genere el video y cópielo al contenedor:

```bash
ffmpeg -f lavfi -i testsrc=duration=10:size=640x480:rate=30 \
  -pix_fmt yuv420p -vcodec mjpeg -q:v 5 Prueba.mp4

docker cp Prueba.mp4 yocto-rust-opencv:/home/root/Prueba.mp4
```

---

## Prueba en QEMU (dentro del contenedor)

```bash
runqemu qemux86-64 vision-image nographic
```

> Si el comando anterior falla, pruebe con la ruta completa:
> ```bash
> runqemu qemux86-64 tmp/deploy/images/qemux86-64/vision-image-qemux86-64.wic.vmdk nographic
> ```

Una vez iniciado el sistema, ingrese como `root` (sin contraseña) y ejecute:

```bash
export DISPLAY=:0
vision_app /home/root/Prueba.mp4
```

---

## Exportar e instalar en VirtualBox

### 1. Copiar la imagen al host

```bash
docker cp yocto-rust-opencv:/workspace/poky/build/tmp/deploy/images/qemux86-64/vision-image-qemux86-64.wic.vmdk \
  ~/ruta/destino/vision-image.vmdk
```

### 2. Corregir permisos del archivo

```bash
sudo chown $USER:$USER ~/ruta/destino/vision-image.vmdk
```

### 3. Crear la máquina virtual en VirtualBox

| Parámetro | Valor recomendado |
|---|---|
| Nombre | YoctoVision |
| Tipo | Linux |
| Versión | Other Linux (64-bit) |
| RAM | 4096 MB |
| CPUs | 2 |
| Disco | Usar disco existente → seleccionar `vision-image.vmdk` |

### 4. Ajustes adicionales antes de iniciar

- **Sistema → Orden de arranque:** dejar solo **Disco duro** marcado
- **Sistema → Procesador:** habilitar PAE/NX
- **Pantalla → Memoria de video:** 16 MB

### 5. Iniciar la VM y ejecutar la aplicación

Al arrancar, el sistema iniciará automáticamente X11. Ingrese como `root` (sin contraseña) y ejecute:

```bash
export DISPLAY=:0
vision_app /home/root/Prueba.mp4
```

---

## Solución de problemas comunes

**`No such file or directory` al ejecutar `vision_app`**  
El linker dinámico no está en `/lib64/`. Solución:
```bash
mkdir -p /lib64
ln -s /lib/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
```

**`No se pudo abrir el archivo de video`**  
Pase la ruta completa al video como argumento:
```bash
vision_app /home/root/Prueba.mp4
```

**Error `VBOX_E_OBJECT_NOT_FOUND` al agregar el disco en VirtualBox**  
El archivo `.vmdk` tiene permisos de `root`. Ejecute:
```bash
sudo chown $USER:$USER ruta/al/archivo.vmdk
```

**Error de `libxcb` durante el build de Yocto**  
Limpie y recompile la receta:
```bash
bitbake -c cleansstate libxcb
bitbake libxcb
```

**El contenedor falla por falta de memoria**  
Aumente el límite en `docker-compose.yml`:
```yaml
mem_limit: 16g
```

**Problemas de permisos en el directorio `workspace`**  
Si Docker creó la carpeta como `root`, corríjala desde el host:
```bash
sudo chown -R $USER:$USER yocto-docker/workspace
```

---

## Referencias

- [Yocto Project Documentation — Kirkstone](https://docs.yoctoproject.org/4.0/)
- [OpenCV](https://opencv.org/)
- [opencv crate para Rust](https://crates.io/crates/opencv)
- [Rust Language](https://rust-lang.org/)

---

## Créditos

| | |
|---|---|
| Curso | Taller de Sistemas Embebidos — TEC Costa Rica |
| Profesor | Dr. Ing. Johan Carvajal Godínez |
| Semestre | I Semestre 2026 |
| Estudiantes | David Leitón y Leonardo Pérez |
