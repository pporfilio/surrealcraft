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
};

TestVoxelData::TestVoxelData(QObject *parent)
    : QObject{parent}
{

}

void TestVoxelData::zeroSize() {
    VoxelData vd = VoxelData(VectorI3D());
    QCOMPARE(vd.dimensions(), VectorI3D());
}


QTEST_MAIN(TestVoxelData)
#include "testvoxeldata.moc"
