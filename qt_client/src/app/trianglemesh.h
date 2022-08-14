#ifndef TRIANGLEMESH_H
#define TRIANGLEMESH_H

#include "voxeldata.h"

#include <QList>

struct Triangle
{
    Triangle(QVector3D p1, QVector3D p2, QVector3D p3, QVector3D color) :
        p1(p1),
        p2(p2),
        p3(p3),
        color(color) {}

    QVector3D p1;
    QVector3D p2;
    QVector3D p3;
    QVector3D color;
};


QList<Triangle> trianglesFromVoxelData(const VoxelData<Voxel> &vd);

#endif // TRIANGLEMESH_H
