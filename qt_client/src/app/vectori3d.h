#ifndef VECTORI3D_H
#define VECTORI3D_H

#include <QVector3D>
#include <QDebug>

class VectorI3D {
public:
    VectorI3D();
    VectorI3D(int x, int y, int z);
    VectorI3D(const VectorI3D &other);

    int x() const;
    int y() const;
    int z() const;

    friend VectorI3D operator+(const VectorI3D v1, const VectorI3D v2);
    friend VectorI3D operator+(const VectorI3D v1, int n);
    friend VectorI3D operator-(const VectorI3D v1, const VectorI3D v2);
    friend VectorI3D operator-(const VectorI3D v1, int n);
    friend VectorI3D operator*(const VectorI3D v1, int n);
    friend VectorI3D operator/(const VectorI3D v1, int n);
    friend bool operator==(const VectorI3D v1, VectorI3D v2);
    friend QDebug operator<<(QDebug dbg, const VectorI3D &v);

    QVector3D asQVector() const;

private:
     int data[3];
};

#endif // VECTORI3D_H
