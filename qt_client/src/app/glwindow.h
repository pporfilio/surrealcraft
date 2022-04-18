#ifndef GLWINDOW_H
#define GLWINDOW_H

#include <QOpenGLWindow>
#include <QMatrix4x4>
#include <QVector3D>

#include <QOpenGLTexture>
#include <QOpenGLShaderProgram>
#include <QOpenGLBuffer>
#include <QOpenGLVertexArrayObject>

//QT_BEGIN_NAMESPACE

//class QOpenGLTexture;
//class QOpenGLShaderProgram;
//class QOpenGLBuffer;
//class QOpenGLVertexArrayObject;

//QT_END_NAMESPACE

class GLWindow : public QOpenGLWindow
{
    Q_OBJECT

public:
    GLWindow();
    ~GLWindow();

    void initializeGL();
    void resizeGL(int w, int h);
    void paintGL();

private:
    const QString m_PROJECTION_MATRIX_NAME = QString("projectionMatrix");
    const QString m_CAMERA_MATRIX_NAME = QString("cameraMatrix");
    const QString m_WORLD_MATRIX_NAME = QString("worldMatrix");
    const QString m_MY_MATRIX_NAME = QString("myMatrix");
    const QString m_LIGHT_POSITION_NAME = QString("lightPosition");
    const QString m_EYE_NAME = QString("eye");
    const QString m_TARGET_NAME = QString("target");

    std::unique_ptr<QOpenGLTexture> m_texture;
    std::unique_ptr<QOpenGLShaderProgram> m_program;
    std::unique_ptr<QOpenGLBuffer> m_vertexBuffer;
    std::unique_ptr<QOpenGLVertexArrayObject> m_vertexArrayObject;

    QMap<QString, int> m_uniformLocations;
    QMap<QString, QMatrix4x4> m_uniformMatrices;
    QMap<QString, QVector3D> m_uniformVectors;

    QList<GLfloat> m_triangle_data;

};

#endif // GLWINDOW_H
