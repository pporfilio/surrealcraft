#include "glwindow.h"

#include <QOpenGLFunctions>
#include <QOpenGLExtraFunctions>
#include <QDebug>
#include <QFile>
#include <QDir>
#include <QCoreApplication>

GLWindow::GLWindow()
{
    m_uniformMatrices.insert(m_WORLD_MATRIX_NAME, QMatrix4x4());
    m_uniformMatrices.insert(m_CAMERA_MATRIX_NAME, QMatrix4x4());
    m_uniformMatrices.insert(m_WORLD_MATRIX_NAME, QMatrix4x4());
    m_uniformMatrices.insert(m_MY_MATRIX_NAME, QMatrix4x4());
    m_uniformVectors.insert(m_LIGHT_POSITION_NAME, QVector3D());
    m_uniformVectors.insert(m_EYE_NAME, QVector3D());
    m_uniformVectors.insert(m_TARGET_NAME, QVector3D());

    m_triangle_data.append(-0.5);
    m_triangle_data.append(-0.5);
    m_triangle_data.append(0.0);
    m_triangle_data.append(0.5);
    m_triangle_data.append(-0.5);
    m_triangle_data.append(0.0);
    m_triangle_data.append(0.0);
    m_triangle_data.append(0.5);
    m_triangle_data.append(0.0);
}

GLWindow::~GLWindow() {
    makeCurrent();
}

void GLWindow::initializeGL() {
    QOpenGLFunctions *f = QOpenGLContext::currentContext()->functions();

    // TODO: handle errors
    qDebug() << QCoreApplication::applicationDirPath();
    QDir applicationRoot(QCoreApplication::applicationDirPath());
    QFile vertexShaderFile(applicationRoot.absoluteFilePath("shaders/scene.vert"));
    vertexShaderFile.open(QIODevice::ReadOnly | QIODevice::Text);
    QFile fragmentShaderFile(applicationRoot.absoluteFilePath("shaders/scene.frag"));
    fragmentShaderFile.open(QIODevice::ReadOnly | QIODevice::Text);

    QTextStream vertexShaderIn(&vertexShaderFile);
    QString vertexShaderSource = vertexShaderIn.readAll();

    QTextStream fragmentShaderIn(&fragmentShaderFile);
    QString fragmentShaderSource = fragmentShaderIn.readAll();

    m_program.reset(new QOpenGLShaderProgram);
    m_program->addShaderFromSourceCode(QOpenGLShader::Vertex, vertexShaderSource);
    m_program->addShaderFromSourceCode(QOpenGLShader::Fragment, fragmentShaderSource);
    m_program->link();
    qInfo() << (m_program->log());


//    m_uniformLocations.insert(m_PROJECTION_MATRIX_NAME, m_program->uniformLocation(m_PROJECTION_MATRIX_NAME));
//    m_uniformLocations.insert(m_CAMERA_MATRIX_NAME, m_program->uniformLocation(m_CAMERA_MATRIX_NAME));
//    m_uniformLocations.insert(m_WORLD_MATRIX_NAME, m_program->uniformLocation(m_WORLD_MATRIX_NAME));
//    m_uniformLocations.insert(m_MY_MATRIX_NAME, m_program->uniformLocation(m_MY_MATRIX_NAME));
//    m_uniformLocations.insert(m_LIGHT_POSITION_NAME, m_program->uniformLocation(m_LIGHT_POSITION_NAME));

    m_vertexArrayObject.reset(new QOpenGLVertexArrayObject);
    if (m_vertexArrayObject->create()) {
        m_vertexArrayObject->bind();
    }

    m_program->bind();
    m_vertexBuffer.reset(new QOpenGLBuffer);
    m_vertexBuffer->create();
    m_vertexBuffer->bind();

    // This copies the contents of the first parameter to the GPU
    m_vertexBuffer->allocate(m_triangle_data.constData(), m_triangle_data.size() * sizeof(GLfloat));

    // This enables `layout(location = 0) in vec4 vertex;` from the shader
    f->glEnableVertexAttribArray(0);

    // This says that layout(location = 0) starts at 0 (nullptr) offset into the VBO
    f->glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);

    m_vertexBuffer->release();
}

void GLWindow::resizeGL(int w, int h) {
//    m_uniformMatrices.insert(m_PROJECTION_MATRIX_NAME, QMatrix4x4());
//    QMatrix4x4 tmp;
//    tmp.perspective(45.0f, GLfloat(w) / h, 0.01f, 100.0f);
//    m_uniformMatrices.insert(m_PROJECTION_MATRIX_NAME, tmp);
}

void GLWindow::paintGL() {
    QOpenGLExtraFunctions *f = QOpenGLContext::currentContext()->extraFunctions();
    f->glClearColor(0, 0, 0, 1);
    f->glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    m_program->bind();
//    QMatrix4x4 tmp;
//    tmp.lookAt(QVector3D(0, 0, -1), QVector3D(0, 0, 0), QVector3D(0, 1, 0));
//    m_uniformMatrices.insert(m_CAMERA_MATRIX_NAME, tmp);
//    m_program->setUniformValue(m_uniformLocations.value(m_PROJECTION_MATRIX_NAME), m_uniformMatrices.value(m_PROJECTION_MATRIX_NAME));
//    m_program->setUniformValue(m_uniformLocations.value(m_CAMERA_MATRIX_NAME), m_uniformMatrices.value(m_CAMERA_MATRIX_NAME));
//    m_program->setUniformValue(m_uniformLocations.value(m_WORLD_MATRIX_NAME), m_uniformMatrices.value(m_WORLD_MATRIX_NAME));
//    m_program->setUniformValue(m_uniformLocations.value(m_MY_MATRIX_NAME), m_uniformMatrices.value(m_MY_MATRIX_NAME));

    f->glDrawArrays(GL_TRIANGLES, 0, m_triangle_data.size() / 3);
}
