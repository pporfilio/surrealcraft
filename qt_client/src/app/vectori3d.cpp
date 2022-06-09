#include "vectori3d.h"

VectorI3D::VectorI3D(): data{0, 0, 0} {};

VectorI3D::VectorI3D(int x, int y, int z): data{x, y, z} {};

VectorI3D::VectorI3D(VectorI3D &other): data{other.x(), other.y(), other.z()} {};

int VectorI3D::x() const { return data[0]; }
int VectorI3D::y() const { return data[1]; }
int VectorI3D::z() const { return data[2]; }

VectorI3D operator+(const VectorI3D v1, const VectorI3D v2)  {
    return VectorI3D(v1.data[0] + v2.data[0], v1.data[1] + v2.data[1], v1.data[2] + v2.data[2]);
}

VectorI3D operator+(const VectorI3D v1, int n)  {
    return VectorI3D(v1.data[0] + n, v1.data[1] + n, v1.data[2] + n);
}

VectorI3D operator-(const VectorI3D v1, const VectorI3D v2)  {
    return VectorI3D(v1.data[0] - v2.data[0], v1.data[1] - v2.data[1], v1.data[2] - v2.data[2]);
}

VectorI3D operator-(const VectorI3D v1, int n)  {
    return VectorI3D(v1.data[0] - n, v1.data[1] - n, v1.data[2] - n);
}

VectorI3D operator*(const VectorI3D v1, int n)  {
    return VectorI3D(v1.data[0] * n, v1.data[1] * n, v1.data[2] * n);
}

VectorI3D operator/(const VectorI3D v1, int n)  {
    return VectorI3D(v1.data[0] / n, v1.data[1] / n, v1.data[2] / n);
}

QVector3D VectorI3D::asQVector() const {
    return QVector3D(data[0], data[1], data[2]);
}
