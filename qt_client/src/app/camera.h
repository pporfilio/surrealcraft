#ifndef CAMERA_H
#define CAMERA_H

#include <QVector3D>

class Camera
{
public:
    Camera();

    float getPitchRad();
    void setPitchDeg(float pitchDeg);
    void addPitchDeg(float pitchDegDelta);

    float getYawRad();
    void setYawDeg(float yawDeg);
    void addYawDeg(float yawDegDelta);

    QVector3D getPosition();
    void setPosition(QVector3D position);
    void addPositionDelta(QVector3D delta);

protected:
    float degToRad(float inDeg);
    float radToDeg(float inRad);
    float fmod(float num, float denom);

private:
    QVector3D m_position;
    float m_pitchRad;
    float m_yawRad;

};

#endif // CAMERA_H
