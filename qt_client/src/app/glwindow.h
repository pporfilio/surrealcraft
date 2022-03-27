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
    const std::string m_PROJECTION_MATRIX_NAME = std::string("projectionMatrix");
    const std::string m_CAMERA_MATRIX_NAME = std::string("cameraMatrix");
    const std::string m_WORLD_MATRIX_NAME = std::string("worldMatrix");
    const std::string m_MY_MATRIX_NAME = std::string("myMatrix");
    const std::string m_LIGHT_POSITION_NAME = std::string("lightPosition");
    const std::string m_EYE_NAME = std::string("eye");
    const std::string m_TARGET_NAME = std::string("target");

    std::unique_ptr<QOpenGLTexture> m_texture;
    std::unique_ptr<QOpenGLShaderProgram> m_program;
    std::unique_ptr<QOpenGLBuffer> m_vertexBuffer;
    std::unique_ptr<QOpenGLVertexArrayObject> m_vertexArrayObject;

    std::unordered_map<std::string, int> m_uniformLocations;
    std::unordered_map<std::string, QMatrix4x4> m_uniformMatrices;
    std::unordered_map<std::string, QVector3D> m_uniformVectors;

};

#endif // GLWINDOW_H
