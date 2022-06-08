#ifndef GLWINDOW_H
#define GLWINDOW_H

#include "inputstate.h"
#include "camera.h"

#include <QOpenGLWindow>
#include <QMatrix4x4>
#include <QVector3D>
#include <QElapsedTimer>
#include <QOpenGLTexture>
#include <QOpenGLShaderProgram>
#include <QOpenGLBuffer>
#include <QOpenGLVertexArrayObject>

class GLWindow : public QOpenGLWindow
{
    Q_OBJECT

public:
    GLWindow();
    ~GLWindow();

    void addCube(QVector3D center, QVector3D color);
    void addVertex(QVector3D v, QVector3D translation, QVector3D color);

    void initializeGL();
    void resizeGL(int w, int h);
    void paintGL();

protected:
    virtual void keyPressEvent(QKeyEvent *ev);
    virtual void keyReleaseEvent(QKeyEvent *ev);
    virtual void mouseDoubleClickEvent(QMouseEvent *ev);
    virtual void mousePressEvent(QMouseEvent *ev);
    virtual void mouseMoveEvent(QMouseEvent *ev);
    virtual void mouseReleaseEvent(QMouseEvent *ev);
    virtual void wheelEvent(QWheelEvent *ev);

    static QVector3D getCameraPositionDelta(const std::unique_ptr<Camera> &camera, const InputState &inputState, float tickDuration);
    static float getCameraYawDeltaDeg(const InputState &inputState, const InputState &previousInputState);
    static float getCameraPitchDeltaDeg(const InputState &inputState, const InputState &previousInputState);

private slots:
    void onFrameSwapped(void);

private:
    const QString m_PROJECTION_MATRIX_NAME = QString("projection");
    const QString m_VIEW_MATRIX_NAME = QString("view");
    const QString m_MODEL_MATRIX_NAME = QString("model");
    const QString m_LIGHT_POSITION_NAME = QString("lightPosition");
    const QString m_EYE_NAME = QString("eye");

    int m_projectionMatrixLocation;
    int m_viewMatrixLocation;
    int m_modelMatrixLocation;
    int m_lightPositionLocation;
    int m_eyeLocation;

    double m_radians = 0.0;

    int m_screenWidth;
    int m_screenHeight;

    QMatrix4x4 m_projectionMatrix = QMatrix4x4();
    QMatrix4x4 m_viewMatrix = QMatrix4x4();
    QMatrix4x4 m_modelMatrix = QMatrix4x4();
    QVector3D m_lightPosition;
    QVector3D m_eyeVector;

    std::unique_ptr<QOpenGLTexture> m_texture;
    std::unique_ptr<QOpenGLShaderProgram> m_program;
    std::unique_ptr<QOpenGLBuffer> m_vertexBuffer;
    std::unique_ptr<QOpenGLVertexArrayObject> m_vertexArrayObject;

    QMap<QString, int> m_uniformLocations;
    QMap<QString, QMatrix4x4> m_uniformMatrices;
    QMap<QString, QVector3D> m_uniformVectors;

    QList<GLfloat> m_triangleData;

    InputState m_currentInputState;
    InputState m_previousInputState;

    std::unique_ptr<Camera> m_camera;

    qint64 m_previousFrameTime;
    QElapsedTimer m_timer;
};

#endif // GLWINDOW_H
