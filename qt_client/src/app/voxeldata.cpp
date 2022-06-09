#include "voxeldata.h"

#include <cstring>
#include <QDebug>


VoxelData::VoxelData(const VectorI3D &dimensions)
{
    int voxelCount = dimensions.x() * dimensions.y() * dimensions.z();
    if (voxelCount > 20 * 1000 * 1000) {
        qDebug() << "Tried to make VoxelData with too many voxels:" << voxelCount;
    }
    m_data.reset(new char[voxelCount * 4]());
}

char* VoxelData::pointerTo(const VectorI3D &indices) const {
    return &m_data[m_voxelBytes * (m_dimensions.x() * m_dimensions.y() * indices.z() + m_dimensions.x() * indices.y() + indices.x())];
}

VoxelData* VoxelData::fromFile(const QString path) {

}

bool VoxelData::hasVoxel(const VectorI3D &indices) const {
    return false;
}

