#include "voxeldata.h"
#include "vectori3d.h"

#include <QObject>
#include <QTest>

class TestVoxelData : public QObject
{
    Q_OBJECT
public:
    explicit TestVoxelData(QObject *parent = nullptr);

private slots:
    void zeroSize();
    void dimensions();
    void elementAccess();
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


QTEST_MAIN(TestVoxelData)
#include "testvoxeldata.moc"
