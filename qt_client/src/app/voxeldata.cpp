#include "voxeldata.h"
#include "vectori3d.h"
#include <QFile>
#include <QCoreApplication>

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

    QDataStream in(&file);
    in >> x >> y >> z;
    if (x * y * z > 1000 * 1000 * 1000) {
        qDebug() << "Got x" << x << "y" << y << "z" << z << "which is too big.";
        return VoxelData<Voxel>(VectorI3D());
    }
    VoxelData<Voxel> voxelData(VectorI3D(x, y, z));
    for (int x_idx = 0; x_idx < x; ++x_idx) {
        for (int y_idx = 0; y_idx < y; ++y_idx) {
            for (int z_idx = 0; z_idx < z; ++z_idx) {
                Voxel *v = voxelData.dataPointerAt(VectorI3D(x_idx, y_idx, z_idx));
                in >> v->m_value >> v->m_r >> v->m_g >> v->m_b;
            }
        }
    }
    return voxelData;
}
