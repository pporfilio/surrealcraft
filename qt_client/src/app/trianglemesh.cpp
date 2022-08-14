#include "trianglemesh.h"

static const float CUBE_SCALE = 0.5;

void addNegativeZ(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D( CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D(-CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
}

void addPositiveZ(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D(-CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D( CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
}

void addNegativeX(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D(-CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
}

void addPositiveX(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D(CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D(CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
}


void addNegativeY(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D( CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
}

void addPositiveY(QList<Triangle> &triangles, const QVector3D &voxelCenter, const QVector3D &color) {
    triangles.append(Triangle(QVector3D( CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              color));
    triangles.append(Triangle(QVector3D(-CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxelCenter,
                              QVector3D(-CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              QVector3D( CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxelCenter,
                              color));
}



QList<Triangle> trianglesFromVoxelData(const VoxelData<Voxel> &vd) {
    QList<Triangle> result;

    int vdDimX = vd.dimensions().x();
    int vdDimY = vd.dimensions().y();
    int vdDimZ = vd.dimensions().z();
    for (int x = 0; x < vdDimX; ++x) {
        for (int y = 0; y < vdDimY; ++y) {
            for (int z = 0; z < vdDimZ; ++z) {
                Voxel voxel = vd.dataAt(VectorI3D(x, y, z));
                if (voxel.m_value != 0) {
                    QVector3D voxelCenter(x, y, z);
                    QVector3D voxelColor(voxel.m_r, voxel.m_g, voxel.m_b);
                    addNegativeX(result, voxelCenter, voxelColor);
                    addPositiveX(result, voxelCenter, voxelColor);
                    addNegativeY(result, voxelCenter, voxelColor);
                    addPositiveY(result, voxelCenter, voxelColor);
                    addNegativeZ(result, voxelCenter, voxelColor);
                    addPositiveZ(result, voxelCenter, voxelColor);
                }
            }
        }
    }
    return result;
}
