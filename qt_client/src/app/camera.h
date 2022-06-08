#ifndef CAMERA_H
#define CAMERA_H

#include <QVector3D>

class Camera
{
public:
    Camera();

    float getPitchRad() const;
    void setPitchDeg(float pitchDeg);
    void setPitchRad(float pitchRad);
    void addPitchDeg(float pitchDegDelta);

    float getYawRad() const;
    void setYawDeg(float yawDeg);
    void setYawRad(float yawRad);
    void addYawDeg(float yawDegDelta);
    QVector3D getLookVector() const;
    QVector3D getUpVector() const;

    void setPosition(QVector3D position);
    void addPositionDelta(QVector3D delta);
    QVector3D getPosition() const;

protected:
    static float degToRad(float inDeg);
    static float radToDeg(float inRad);
    static float fmod(float num, float denom);

private:
    QVector3D m_position;
    float m_pitchRad;
    float m_yawRad;

};

#endif // CAMERA_H
