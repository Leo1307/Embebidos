# Fix para Kirkstone: asegurar que xcb-proto-native se compile primero
# y que la ruta de los archivos XCB sea correcta.
# Sin esto, make falla con: "No rule to make target '//usr/share/xcb/'"
DEPENDS += "xcb-proto-native"

XCBPROTO = "${STAGING_DIR_NATIVE}${prefix}/share/xcb"
