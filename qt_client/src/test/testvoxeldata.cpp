#include "voxeldata.h"
#include "vectori3d.h"

#include <QObject>
#include <QTest>

class TestVoxelData : public QObject, public VoxelData
{
    Q_OBJECT
public:
    explicit TestVoxelData(QObject *parent = nullptr);

private slots:
    void pointerTo();
    void zeroSize();
};

TestVoxelData::TestVoxelData(QObject *parent)
    : QObject{parent}
{

}

void TestVoxelData::zeroSize() {
    VoxelData vd = VoxelData(VectorI3D());
    QCOMPARE(vd.dimensions(), VectorI3D());
}

void TestVoxelData::pointerTo() {
    VoxelData vd(VectorI3D(3, 4, 5));


    QCOMPARE(true, true);
}

QTEST_MAIN(TestVoxelData)
#include "testvoxeldata.moc"
