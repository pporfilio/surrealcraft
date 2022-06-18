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
};

TestVoxelData::TestVoxelData(QObject *parent)
    : QObject{parent}
{

}

void TestVoxelData::zeroSize() {
    VoxelData vd = VoxelData(VectorI3D());
    QCOMPARE(vd.dimensions(), VectorI3D());
}

void TestVoxelData::dimensions() {
    VoxelData vd = VoxelData(VectorI3D(1, 2, 3));
    QCOMPARE(vd.dimensions(), VectorI3D(1, 2, 3));
}


QTEST_MAIN(TestVoxelData)
#include "testvoxeldata.moc"
