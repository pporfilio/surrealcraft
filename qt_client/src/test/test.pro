QT += widgets testlib

INCLUDEPATH += ../app
DEPENDPATH += ../app

HEADERS = ../app/vectori3d.h \
          ../app/voxeldata.h

SOURCES = ../app/vectori3d.cpp \
          testvoxeldata.cpp



CONFIG(debug, debug|release) {
    DESTDIR = debug
} else {
    DESTDIR = release
}

# install
target.path = $$[DESTDIR]/test
INSTALLS += target
