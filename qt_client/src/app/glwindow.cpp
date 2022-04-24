#include "glwindow.h"

#include <QOpenGLFunctions>
#include <QOpenGLExtraFunctions>
#include <QDebug>
#include <QFile>
#include <QDir>
#include <QCoreApplication>
#include <QTime>

void GLWindow::addVertex(QVector3D v) {

    m_triangleData.append(v.x());
    m_triangleData.append(v.y());
    m_triangleData.append(v.z());

}

GLWindow::GLWindow()
{

    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));

    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f));

    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f));

    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));

    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f));

    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f));
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f));
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


    m_projectionMatrixLocation = m_program->uniformLocation(m_PROJECTION_MATRIX_NAME);
    m_modelMatrixLocation = m_program->uniformLocation(m_MODEL_MATRIX_NAME);
    m_viewMatrixLocation = m_program->uniformLocation(m_VIEW_MATRIX_NAME);

    m_vertexArrayObject.reset(new QOpenGLVertexArrayObject);
    if (m_vertexArrayObject->create()) {
        m_vertexArrayObject->bind();
    }

    m_program->bind();
    m_vertexBuffer.reset(new QOpenGLBuffer);
    m_vertexBuffer->create();
    m_vertexBuffer->bind();

    // This copies the contents of the first parameter to the GPU
    m_vertexBuffer->allocate(m_triangleData.constData(), m_triangleData.size() * sizeof(GLfloat));

    // This enables `layout(location = 0) in vec4 vertex;` from the shader
    f->glEnableVertexAttribArray(0);

    // This says that layout(location = 0) starts at 0 (nullptr) offset into the VBO
    f->glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), nullptr);

    m_vertexBuffer->release();
}

void GLWindow::resizeGL(int w, int h) {
//    QMatrix4x4 tmp;
//    tmp.perspective(45.0f, GLfloat(w) / h, 0.01f, 100.0f);
//    m_uniformMatrices.insert(m_PROJECTION_MATRIX_NAME, tmp);
}

void GLWindow::paintGL() {
    QOpenGLExtraFunctions *f = QOpenGLContext::currentContext()->extraFunctions();
    f->glClearColor(0, 0, 0, 1);
    f->glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    m_program->bind();

    m_modelMatrix.translate(QVector3D(0.0, 0.0, 0.0));
    m_viewMatrix.translate(QVector3D(0.0, 0.0, -3.0));
    m_projectionMatrix.perspective(45.0, 800.0 / 600.0, 0.1, 100.0);

    qDebug() << m_modelMatrix;
    qDebug() << m_viewMatrix;
    qDebug() << m_projectionMatrix;

    m_program->setUniformValue(m_modelMatrixLocation, m_modelMatrix);
    m_program->setUniformValue(m_viewMatrixLocation, m_viewMatrix);
    m_program->setUniformValue(m_projectionMatrixLocation, m_projectionMatrix);

    qDebug() << QTime::currentTime();

    f->glDrawArrays(GL_TRIANGLES, 0, m_triangleData.size() / 3);
}
