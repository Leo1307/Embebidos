#!/bin/bash
# ==============================================================
# setup-yocto.sh (versión para repositorio)
# Ejecutar DENTRO del contenedor Docker después del primer inicio
# Clona repositorios base y copia las capas personalizadas
# ==============================================================

set -e

WORKSPACE="/workspace"
KIRKSTONE_BRANCH="kirkstone"

echo "============================================="
echo " Configurando Yocto Kirkstone + Rust + OpenCV"
echo " Proyecto Embebidos - TEC"
echo "============================================="

sudo chown -R yocto:yocto "$WORKSPACE" 2>/dev/null || true
cd "$WORKSPACE"

# 1. Clonar Poky
if [ ! -d "poky" ]; then
    echo "[1/4] Clonando Poky (Kirkstone)..."
    git clone -b "$KIRKSTONE_BRANCH" git://git.yoctoproject.org/poky.git
else
    echo "[1/4] Poky ya existe, saltando..."
fi

# 2. Clonar meta-openembedded
if [ ! -d "meta-openembedded" ]; then
    echo "[2/4] Clonando meta-openembedded (Kirkstone)..."
    git clone -b "$KIRKSTONE_BRANCH" git://git.openembedded.org/meta-openembedded
else
    echo "[2/4] meta-openembedded ya existe, saltando..."
fi

# 3. Clonar meta-rust-bin
if [ ! -d "meta-rust-bin" ]; then
    echo "[3/4] Clonando meta-rust-bin..."
    git clone https://github.com/pmetaxas/meta-rust-bin.git
else
    echo "[3/4] meta-rust-bin ya existe, saltando..."
fi

# 4. Copiar capas personalizadas desde el directorio montado (yocto-docker/layers/)
if [ -d "/workspace/layers/meta-rust-opencv" ]; then
    echo "[4/4] Copiando capa meta-rust-opencv..."
    cp -r /workspace/layers/meta-rust-opencv poky/
fi

if [ -d "/workspace/layers/meta-local" ]; then
    echo "[4/4] Copiando capa meta-local..."
    cp -r /workspace/layers/meta-local poky/
fi

# 5. Inicializar entorno de build y agregar capas
echo ""
echo "Inicializando entorno de build..."
cd "$WORKSPACE/poky"
source oe-init-build-env

echo "Agregando capas a bblayers.conf..."
bitbake-layers add-layer /workspace/meta-openembedded/meta-oe 2>/dev/null || echo "  meta-oe ya agregada"
bitbake-layers add-layer /workspace/meta-openembedded/meta-python 2>/dev/null || echo "  meta-python ya agregada"
bitbake-layers add-layer /workspace/meta-openembedded/meta-multimedia 2>/dev/null || echo "  meta-multimedia ya agregada"
bitbake-layers add-layer /workspace/poky/meta-rust-opencv 2>/dev/null || echo "  meta-rust-opencv ya agregada"
bitbake-layers add-layer /workspace/meta-rust-bin 2>/dev/null || echo "  meta-rust-bin ya agregada"
bitbake-layers add-layer /workspace/poky/meta-local 2>/dev/null || echo "  meta-local ya agregada"

echo ""
echo "Capas configuradas:"
bitbake-layers show-layers

# Configurar local.conf
echo ""
echo "Configurando local.conf..."
if ! grep -q "# === Proyecto Rust OpenCV ===" conf/local.conf 2>/dev/null; then
    cat >> conf/local.conf << 'CONFEOF'

# === Proyecto Rust OpenCV ===
MACHINE = "qemux86-64"
BB_NUMBER_THREADS ?= "${@oe.utils.cpu_count()}"
PARALLEL_MAKE ?= "-j ${@oe.utils.cpu_count()}"
IMAGE_ROOTFS_EXTRA_SPACE = "1048576"
# IMAGE_INSTALL:append = " rust-opencv"
CONFEOF
    echo "   local.conf actualizado."
else
    echo "   local.conf ya configurado."
fi

echo ""
echo "============================================="
echo " ¡Configuración completada!"
echo "============================================="
echo ""
echo "Próximos pasos (dentro del contenedor):"
echo ""
echo "  1. Compilar libxcb (verificar fix):"
echo "     bitbake -c cleansstate libxcb"
echo "     bitbake libxcb"
echo ""
echo "  2. Compilar app Rust (cuando la receta esté lista):"
echo "     bitbake rust-opencv"
echo ""
echo "  3. Generar imagen completa:"
echo "     bitbake core-image-sato"
echo ""
echo "  Para recargar entorno en futuras sesiones:"
echo "     cd /workspace/poky && source oe-init-build-env"
echo ""
