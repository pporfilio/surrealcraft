#ifndef CAMERA_H
#define CAMERA_H

#include <QVector3D>

class Camera
{
public:
    Camera();

    float getPitchRad();
    void setPitchDeg(float pitch);
    float getYawRad();
    void setYawDeg(float yaw);

    void moveDelta(QVector3D newPosition);

private:

    QVector3D m_position;
    float m_pitch;
    float m_yaw;

};

#endif // CAMERA_H
