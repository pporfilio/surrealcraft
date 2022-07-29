#include "voxeldata.h"
#include "vectori3d.h"

#include <QObject>
#include <QTest>
#include <QTemporaryFile>

class TestVoxelData : public QObject
{
    Q_OBJECT
public:
    explicit TestVoxelData(QObject *parent = nullptr);

private slots:
    void zeroSize();
    void dimensions();
    void elementAccess();
    void testReadVoxel();
};

TestVoxelData::TestVoxelData(QObject *parent)
    : QObject{parent}
{

}

void TestVoxelData::zeroSize() {
    VoxelData vd = VoxelData<int>(VectorI3D());
    QCOMPARE(vd.dimensions(), VectorI3D());
}

void TestVoxelData::dimensions() {
    VoxelData vd = VoxelData<int>(VectorI3D(1, 2, 3));
    QCOMPARE(vd.dimensions(), VectorI3D(1, 2, 3));
}

void TestVoxelData::elementAccess() {
    VoxelData vd = VoxelData<int>(VectorI3D(3, 4, 5));
    for (int x = 0; x < 3; ++x) {
        for (int y = 0; y < 4; ++y) {
            for (int z = 0; z < 5; ++z) {
                 vd.setDataAt(VectorI3D(x, y, z), x * y * z);
            }
        }
    }

    for (int x = 0; x < 3; ++x) {
        for (int y = 0; y < 4; ++y) {
            for (int z = 0; z < 5; ++z) {
                 QCOMPARE(vd.dataAt(VectorI3D(x, y, z)), x * y * z);
//                 QCOMPARE(*vd.dataPointerAt(VectorI3D(x, y, z)), x * y * z);
            }
        }
    }
}

void TestVoxelData::testReadVoxel() {
    QTemporaryFile file;
    if (file.open()) {
        qint32 x = 3;
        qint32 y = 4;
        qint32 z = 5;
        writeBigEndian(file, reinterpret_cast<char *>(&x), sizeof(x));
        writeBigEndian(file, reinterpret_cast<char *>(&y), sizeof(y));
        writeBigEndian(file, reinterpret_cast<char *>(&z), sizeof(z));
        for (int z = 0; z < 5; ++z) {
            for (int y = 0; y < 4; ++y) {
                for (int x = 0; x < 3; ++x) {
                    Voxel v(x * y * z, x / 3.0, y / 4.0, z / 5.0);
                    writeBigEndian(file, reinterpret_cast<char *>(&v.m_value), sizeof(v.m_value));
                    writeBigEndian(file, reinterpret_cast<char *>(&v.m_r), sizeof(v.m_r));
                    writeBigEndian(file, reinterpret_cast<char *>(&v.m_g), sizeof(v.m_g));
                    writeBigEndian(file, reinterpret_cast<char *>(&v.m_b), sizeof(v.m_b));
                }
            }
        }
        file.close();

        qDebug() << QDir::tempPath();
        VoxelData<Voxel> vd = voxelsFromFile(file.fileName());
        QCOMPARE(vd.dimensions(), VectorI3D(3, 4, 5));
        for (int x = 0; x < 3; ++x) {
            for (int y = 0; y < 4; ++y) {
                for (int z = 0; z < 5; ++z) {
                    Voxel v = vd.dataAt(VectorI3D(x, y, z));
                    QCOMPARE(v.m_value, x * y * z);
                    QCOMPARE(v.m_r, (float)(x / 3.0));
                    QCOMPARE(v.m_g, (float)(y / 4.0));
                    QCOMPARE(v.m_b, (float)(z / 5.0));
                }
            }
        }
    } else {
        Q_ASSERT(false);
    }
}


QTEST_MAIN(TestVoxelData)
#include "testvoxeldata.moc"
