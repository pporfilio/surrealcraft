#include "camera.h"

#include <QtMath>

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
    return num - (num * (num / denom));
}

float Camera::getPitchRad() {
    return m_pitchRad;
}

void Camera::setPitchDeg(float pitchDeg) {
    m_pitchRad = degToRad(pitchDeg);
    if (m_pitchRad > M_2_PI) {
        m_pitchRad = fmod(m_pitchRad, M_2_PI);
    }
}

void Camera::addPitchDeg(float pitchDegDelta) {
    setPitchDeg(m_pitchRad + degToRad(pitchDegDelta));
}

float Camera::getYawRad() {
    return m_yawRad;
}

void Camera::setYawDeg(float yawDeg) {
    m_yawRad = degToRad(yawDeg);
    if (m_yawRad > M_2_PI) {
        m_yawRad = fmod(m_yawRad, M_2_PI);
    }
}

void Camera::addYawDeg(float yawDegDelta) {
    setYawDeg(m_yawRad + degToRad(yawDegDelta));
}

QVector3D Camera::getPosition() {
    return m_position;
}

void Camera::setPosition(QVector3D position) {
    m_position = position;
}

void Camera::addPositionDelta(QVector3D delta) {
    m_position += delta;
}
