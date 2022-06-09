#ifndef TRIANGLEMESH_H
#define TRIANGLEMESH_H

#include "voxeldata.h"

#include <QList>

class TriangleMesh
{
public:
    TriangleMesh();

    TriangleMesh(VoxelData voxels);

    QList<float> toVertexData();
};

#endif // TRIANGLEMESH_H
