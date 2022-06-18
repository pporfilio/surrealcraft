QT += opengl

# file_copies based on https://stackoverflow.com/a/54162789
# see `COPIES += shaders` below
CONFIG += file_copies

CONFIG(debug, debug|release) {
    DESTDIR = debug
} else {
    DESTDIR = release
}

INCLUDEPATH += src/app

DEPENDPATH += src/app

HEADERS = src/app/glwindow.h \
          src/app/camera.h \
          src/app/inputstate.h \
          src/app/trianglemesh.h \
          src/app/vectori3d.h \
          src/app/voxeldata.h

SOURCES = src/app/glwindow.cpp \
          src/app/camera.cpp \
          src/app/inputstate.cpp \
          src/app/main.cpp \
          src/app/trianglemesh.cpp \
          src/app/vectori3d.cpp \
          src/app/voxeldata.cpp \
          src/test/testvoxeldata.cpp

COPIES += shaders

shaders.files = src/shaders/scene.vert \
                src/shaders/scene.frag

shaders.path = $$DESTDIR/shaders
