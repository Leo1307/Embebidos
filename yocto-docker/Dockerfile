# Usamos Ubuntu 22.04 como base (Yocto recomienda versiones LTS)
FROM ubuntu:22.04

# Evita interacciones interactivas durante la instalación
ENV DEBIAN_FRONTEND=noninteractive

# Actualizamos e instalamos dependencias de Yocto
RUN apt update && apt install -y \
    build-essential \
    chrpath \
    cpio \
    diffstat \
    file \
    gawk \
    gcc \
    git \
    gzip \
    libncurses5-dev \
    libssl-dev \
    locales \
    python3 \
    python3-pip \
    python3-venv \
    wget \
    xz-utils \
    zstd \
    && rm -rf /var/lib/apt/lists/*

# Configurar locale (Yocto lo necesita)
RUN locale-gen en_US.UTF-8
ENV LANG en_US.UTF-8
ENV LC_ALL en_US.UTF-8

# Instalar Rust (usando rustup)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Establecer directorio de trabajo
WORKDIR /workspace

# Comando por defecto: bash
CMD ["/bin/bash"]

