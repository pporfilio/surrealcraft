#include "camera.h"

#include <QtMath>
#include <QDebug>

Camera::Camera()
{

}

float Camera::degToRad(float inDeg) {
    return inDeg * M_2_PI / 180;
}

float Camera::radToDeg(float inRad) {
    return inRad * 180.0 / M_2_PI;
}

float Camera::fmod(float num, float denom) {
    return num - (num * (floor(num / denom)));
}

float Camera::getPitchRad() const {
    return m_pitchRad;
}

void Camera::setPitchDeg(float pitchDeg) {
    setPitchRad(degToRad(pitchDeg));
}

void Camera::setPitchRad(float pitchRad) {
    m_pitchRad = pitchRad;
    qDebug() << "Pitch set to " << m_yawRad;
    if (m_pitchRad < 0) {
        m_pitchRad = M_2_PI - fmod(-1 * m_pitchRad, M_2_PI);
    }
    if (m_pitchRad > M_2_PI) {
        m_pitchRad = fmod(m_pitchRad, M_2_PI);
    }
}

void Camera::addPitchDeg(float pitchDegDelta) {
    setPitchRad(m_pitchRad + degToRad(pitchDegDelta));
}

float Camera::getYawRad() const {
    return m_yawRad;
}

void Camera::setYawDeg(float yawDeg) {
    setYawRad(degToRad(yawDeg));
}

void Camera::setYawRad(float yawRad) {
    m_yawRad = yawRad;
    qDebug() << "Yaw set to " << m_yawRad;
    if (m_yawRad < 0) {
        m_yawRad = M_2_PI - fmod(-1 * m_yawRad, M_2_PI);
    }
    if (m_yawRad > M_2_PI) {
        m_yawRad = fmod(m_yawRad, M_2_PI);
    }
}

void Camera::addYawDeg(float yawDegDelta) {
    setYawRad(m_yawRad + degToRad(yawDegDelta));
}

QVector3D Camera::getLookVector() const {
    return QVector3D(cos(getYawRad()) * cos(getPitchRad()),
                     sin(getPitchRad()),
                     sin(getYawRad()) * cos(getPitchRad()));
}

void Camera::setPosition(QVector3D position) {
    m_position = position;
}

void Camera::addPositionDelta(QVector3D delta) {
    m_position += delta;
}

QVector3D Camera::getPosition() const {
    return m_position;
}
