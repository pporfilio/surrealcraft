#include "camera.h"

#include <QtMath>
#include <QDebug>

Camera::Camera()
{

}

float Camera::degToRad(float inDeg) {
    return inDeg * 2 * M_PI / 180;
}

float Camera::radToDeg(float inRad) {
    return inRad * 180.0 / 2 * M_PI;
}

float Camera::fmod(float num, float denom) {
    return num - (denom * (floor(num / denom)));
}

float Camera::getPitchRad() const {
    return m_pitchRad;
}

void Camera::setPitchDeg(float pitchDeg) {
    setPitchRad(degToRad(pitchDeg));
}

void Camera::setPitchRad(float pitchRad) {
    if (m_pitchRad == pitchRad) {
        return;
    }
    m_pitchRad = pitchRad;
    qDebug() << "Pitch set to " << m_yawRad;
    if (m_pitchRad < 0) {
        m_pitchRad = 2 * M_PI + fmod(m_pitchRad, -1 * 2 * M_PI);
    }
    if (m_pitchRad > 2 * M_PI) {
        m_pitchRad = fmod(m_pitchRad, 2 * M_PI);
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
    qDebug() << "Requesting to set yaw Rad to" << yawRad;
    if (m_yawRad == yawRad) {
        return;
    }
    m_yawRad = yawRad;
    qDebug() << "Yaw set to " << m_yawRad;
    if (m_yawRad < 0) {
        qDebug() << "less than 0";
        qDebug() << "fmod" << fmod(m_yawRad, -1 * 2 * M_PI);
        qDebug() << "2 * M_PI" << 2 * M_PI;
        m_yawRad = 2 * M_PI + fmod(m_yawRad, -1 * 2 * M_PI);
    }
    if (m_yawRad > 2 * M_PI) {
        qDebug() << "greater than 2pi";
        qDebug() << "less than 0";
        qDebug() << "fmod" << fmod(m_yawRad, -1 * 2 * M_PI);
        qDebug() << "2 * M_PI" << 2 * M_PI;
        m_yawRad = fmod(m_yawRad, 2 * M_PI);
    }
}

void Camera::addYawDeg(float yawDegDelta) {
    if (yawDegDelta != 0) {
        qDebug() << "yawDegDelta" << yawDegDelta << "yawRadDelta" << degToRad(yawDegDelta);
    }
    setYawRad(m_yawRad + degToRad(yawDegDelta));
}

QVector3D Camera::getLookVector() const {
    QVector3D unormalized(cos(getYawRad()) * cos(getPitchRad()),
                     sin(getPitchRad()),
                     sin(getYawRad()) * cos(getPitchRad()));
    return unormalized.normalized();
}

QVector3D Camera::getUpVector() const {
    return QVector3D(0.0, 1.0, 0.0);
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
