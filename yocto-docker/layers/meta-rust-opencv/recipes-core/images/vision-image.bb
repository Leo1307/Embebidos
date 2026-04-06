SUMMARY = "Imagen minima con app Rust + OpenCV"
DESCRIPTION = "Imagen Linux a la medida partiendo de core-image-minimal \
               con soporte para OpenCV y la app de deteccion de rostros"

inherit core-image

IMAGE_FEATURES += "x11-base"

IMAGE_INSTALL:append = " \
    rust-opencv \
    opencv \
    ffmpeg \
    libxcb \
    gtk+3 \
    xterm \
    matchbox-wm \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-libav \
"

IMAGE_FSTYPES:append = " wic.vmdk"

IMAGE_ROOTFS_EXTRA_SPACE = "1048576"

WKS_FILE = "directdisk.wks"
