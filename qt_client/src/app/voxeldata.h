#ifndef VOXELDATA_H
#define VOXELDATA_H


#include "vectori3d.h"
#include <QVector3D>

// Origin front-bottom-left
// Array indexing a[z][y][x]
// Array rate of change a[slowest][middle][fastest]
// Voxel coordinates (x, y, z)

// TODO: Add test that reading and writing the data to disk
// has the correct array ordering.

class VoxelData
{
public:
    VoxelData(const VectorI3D &dimensions);

    static VoxelData* fromFile(const QString path);

    VectorI3D dimensions() const;
    bool hasVoxel(const VectorI3D &indices) const;

    QVector3D voxelColor(const VectorI3D &indices) const;

    void setVoxelValue(VectorI3D indices, char value);
    void setVoxelColor(VectorI3D indices, QVector3D color);

    char* copyOfData() const;

private:
    char* pointerTo(const VectorI3D &indices) const;
    std::unique_ptr<char[]> m_data;
    VectorI3D m_dimensions;
    static const int m_voxelBytes = 4;
};

#endif // VOXELDATA_H
