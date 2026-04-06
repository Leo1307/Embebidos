SUMMARY = "Deteccion de rostros con Rust y OpenCV"
DESCRIPTION = "App demostrativa que detecta rostros en video usando Haar Cascades"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = " \
    file://vision_app \
    file://Prueba.mp4 \
    file://Prueba2.mp4 \
"

DEPENDS = "opencv ffmpeg"

RDEPENDS:${PN} = "opencv ffmpeg libxcb"

INSANE_SKIP:${PN} = "ldflags already-stripped file-rdeps"

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${WORKDIR}/vision_app ${D}${bindir}/vision_app

    install -d ${D}/home/root/vision_app
    install -m 0644 ${WORKDIR}/Prueba.mp4 ${D}/home/root/vision_app/
    install -m 0644 ${WORKDIR}/Prueba2.mp4 ${D}/home/root/vision_app/
}

FILES:${PN} = "${bindir}/vision_app /home/root/vision_app/*"
