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
        QDataStream out(&file);
        out << (quint32)3 << (quint32)4 << (quint32)5;
        for (int x = 0; x < 3; ++x) {
            for (int y = 0; y < 4; ++y) {
                for (int z = 0; z < 5; ++z) {
                    out << (quint32)(x * y * z) << (float)(x / 3.0) << (float)(y / 4.0) << (float)(z / 5.0);
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
