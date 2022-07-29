#include "voxeldata.h"
#include "vectori3d.h"
#include <QFile>
#include <QCoreApplication>

// Set the system's endianness.
//#define BIG_ENDIAN 1

#ifdef BIG_ENDIAN
void readBigEndian(QFile &file, char *destination, qint64 bytes) {
    file.read(destination, bytes);
}
#else
void readBigEndian(QFile &file, char *destination, qint64 bytes) {
    char buffer[bytes];
    file.read(&buffer[0], bytes);
    for (qint64 i = 0; i < bytes; ++i) {
        destination[i] = buffer[bytes - 1 - i];
    }
}
#endif

#ifdef BIG_ENDIAN
void writeBigEndian(QFile &file, char *source, qint64 bytes) {
    file.write(source, bytes);
}
#else
void writeBigEndian(QFile &file, char *source, qint64 bytes) {
    char buffer[bytes];
    for (qint64 i = 0; i < bytes; ++i) {
        buffer[i] = source[bytes - 1 - i];
    }
    file.write(&buffer[0], bytes);
}
#endif

VoxelData<Voxel> voxelsFromFile(QString path) {
    QFile file(path);
    if (!file.open(QIODevice::ReadOnly)) {
        qDebug() << "Application path" << QCoreApplication::applicationDirPath();
        qDebug() << "Could not read file" << path;
        return VoxelData<Voxel>(VectorI3D());
    }
    qint32 x;
    qint32 y;
    qint32 z;

    readBigEndian(file, reinterpret_cast<char *>(&x), sizeof(x));
    readBigEndian(file, reinterpret_cast<char *>(&y), sizeof(y));
    readBigEndian(file, reinterpret_cast<char *>(&z), sizeof(z));
    qDebug() << "Read voxel dimensions" << x << y << z;
    if (x < 0 || y < 0 || z < 0) {
        qDebug() << "Got negative value for x, y, or z";
        return VoxelData<Voxel>(VectorI3D());
    }
    if (static_cast<qint64>(x) * static_cast<qint64>(y) * static_cast<qint64>(z) > 1000 * 1000 * 1000) {
        qDebug() << "Got x" << x << "y" << y << "z" << z << "which is too big.";
        return VoxelData<Voxel>(VectorI3D());
    }

    VoxelData<Voxel> voxelData(VectorI3D(x, y, z));
    for (int z_idx = 0; z_idx < z; ++z_idx) {
        for (int y_idx = 0; y_idx < y; ++y_idx) {
            for (int x_idx = 0; x_idx < x; ++x_idx) {
                Voxel *v = voxelData.dataPointerAt(VectorI3D(x_idx, y_idx, z_idx));
                readBigEndian(file, reinterpret_cast<char *>(&v->m_value), sizeof(v->m_value));
                readBigEndian(file, reinterpret_cast<char *>(&v->m_r), sizeof(v->m_r));
                readBigEndian(file, reinterpret_cast<char *>(&v->m_g), sizeof(v->m_g));
                readBigEndian(file, reinterpret_cast<char *>(&v->m_b), sizeof(v->m_b));
            }
        }
    }
    return voxelData;
}
