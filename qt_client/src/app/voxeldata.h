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

struct Voxel {
   Voxel(int32_t value, float r, float g, float b): m_value(value), m_r(r), m_g(g), m_b(b) {}
   Voxel(): m_value(0), m_r(0), m_g(0), m_b(0) {}

   int m_value;
   float m_r;
   float m_g;
   float m_b;
};


template <class T>
class VoxelData
{
public:
    VoxelData(const VectorI3D &dimensions): m_dimensions(dimensions) {
        m_voxelCount = m_dimensions.x() * m_dimensions.y() * m_dimensions.z();
        if (m_voxelCount * sizeof(T) > 1000 * 1000 * 1000) {
            qDebug() << "Tried to make VoxelData with too many voxels: " << m_voxelCount << " bytes per voxel " << sizeof(T);
        }
        qDebug() << "Initializing voxel data with" << m_voxelCount << "voxels.";
        m_data.reset(new T[m_voxelCount]());
    }

    static VoxelData* fromFile(const QString path);

    VectorI3D dimensions() const {
        return m_dimensions;
    }

    void setDataAt(const VectorI3D indices, const T &value) {
        qDebug() << "setting" << indices;
        m_data[arrayOffset(indices)] = value;
    }

    T dataAt(const VectorI3D &indices) const {
        qDebug() << "Accessing data at" << indices;
//        return Voxel();
        return m_data[arrayOffset(indices)];
    }

    T* dataPointerAt(const VectorI3D &indices) const {
        return &m_data[arrayOffset(indices)];
    }


private:
    uint arrayOffset(const VectorI3D &indices) const {
        uint offset = static_cast<uint>(m_dimensions.x() * m_dimensions.y() * indices.z() + m_dimensions.x() * indices.y() + indices.x());
        if (offset >= m_voxelCount) {
            qDebug() << "Attempt to access voxel" << offset << "but array has" << m_voxelCount << "elements.";
            throw std::out_of_range("Attempt to access out of bounds voxel.");
        }
        return offset;
    }

    std::unique_ptr<T[]> m_data;
    VectorI3D m_dimensions;
    uint m_voxelCount;
};

#endif // VOXELDATA_H

