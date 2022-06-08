#include "glwindow.h"

#include <QOpenGLFunctions>
#include <QOpenGLExtraFunctions>
#include <QDebug>
#include <QFile>
#include <QDir>
#include <QCoreApplication>
#include <QTime>
#include <QtMath>
#include <QKeyEvent>
#include <QWheelEvent>


GLWindow::GLWindow()
{
    resize(800, 600);

    m_timer.start();
    m_previousFrameTime = m_timer.msecsSinceReference();

    QObject::connect(this, SIGNAL(frameSwapped()), this, SLOT(onFrameSwapped()));

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

void GLWindow::addVertex(QVector3D v) {
    m_triangleData.append(v.x());
    m_triangleData.append(v.y());
    m_triangleData.append(v.z());
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

    m_camera.reset(new Camera);
}

void GLWindow::resizeGL(int w, int h) {
    m_screenWidth = w;
    m_screenHeight = h;
}

QVector3D GLWindow::getCameraPositionDelta(const std::unique_ptr<Camera> &camera, const InputState &inputState, float tickDuration) {
    float movementScale = 1;
    float deltaForward = 0;
    float deltaUp = 0;
    float deltaRight = 0;
    if (inputState.keyPressed(Qt::Key_W)) {
        deltaForward += tickDuration * movementScale;
    }
    if (inputState.keyPressed(Qt::Key_S)) {
        deltaForward -= tickDuration * movementScale;
    }
    if (inputState.keyPressed(Qt::Key_A)) {
        deltaRight -= tickDuration * movementScale;
    }
    if (inputState.keyPressed(Qt::Key_D)) {
        deltaRight += tickDuration * movementScale;
    }
    if (inputState.keyPressed(Qt::Key_Q)) {
        deltaUp -= tickDuration * movementScale;
    }
    if (inputState.keyPressed(Qt::Key_E)) {
        deltaUp += tickDuration * movementScale;
    }

    return deltaForward * camera->getLookVector() + \
            deltaUp * camera->getUpVector() + \
            deltaRight * QVector3D::crossProduct(camera->getLookVector(),
                                                 camera->getUpVector()).normalized();
}

float GLWindow::getCameraYawDeltaDeg(const InputState &inputState,
                                     const InputState &previousInputState) {
    float rotateScale = 0.3;
    if (!previousInputState.m_mousePositionSet) {
        return 0;
    }
    return (inputState.m_mousePosition.x() - previousInputState.m_mousePosition.x()) * rotateScale;
}

float GLWindow::getCameraPitchDeltaDeg(const InputState &inputState,
                                       const InputState &previousInputState) {
    float rotateScale = 0.1;
    if (!previousInputState.m_mousePositionSet) {
        return 0;
    }
    // Negative one because y is bigger at the bottom than the top of the window.
    return -1 * (inputState.m_mousePosition.y() - previousInputState.m_mousePosition.y()) * rotateScale;
}

void GLWindow::paintGL() {

    m_timer.start();
    qint64 currentTime = m_timer.msecsSinceReference();
    // I think casting to float here does not lose precision because the difference in
    // time should be on the order of 1000s at most.
    float tickDuration = static_cast<float>(currentTime - m_previousFrameTime) / 1000.0;
    m_previousFrameTime = currentTime;

    // Make a local copy of the current and previous input state
    InputState inputState(m_currentInputState);
    InputState previousInputState(m_previousInputState);
//    qDebug() << "-----------";
//    qDebug() << previousInputState.m_mousePosition;
//    qDebug() << inputState.m_mousePosition;
//    qDebug() << "-----------";


    // Copy the current input state for next frame. Continue to update the same m_currentInputState
    // as new events come in.
    m_previousInputState = InputState(m_currentInputState);

    QOpenGLExtraFunctions *f = QOpenGLContext::currentContext()->extraFunctions();
    f->glClearColor(0, 0, 0, 1);
    f->glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    m_program->bind();

    m_modelMatrix.translate(QVector3D(0.0, 0.0, 0.0));

    m_viewMatrix.setToIdentity();

    m_camera->addPositionDelta(getCameraPositionDelta(m_camera, inputState, tickDuration));
    m_camera->addPitchDeg(getCameraPitchDeltaDeg(inputState, previousInputState));
    m_camera->addYawDeg(getCameraYawDeltaDeg(inputState, previousInputState));

    m_viewMatrix.lookAt(m_camera->getPosition(), m_camera->getPosition() + m_camera->getLookVector(), m_camera->getUpVector());

    // Avoid divide by zero
    if (m_screenHeight < 1) {
        m_screenHeight = 1;
    }
    m_projectionMatrix.setToIdentity();
    m_projectionMatrix.perspective(45.0, static_cast<float>(m_screenWidth) / m_screenHeight, 0.1, 100.0);

//    qDebug() << m_viewMatrix;
//    qDebug() << m_projectionMatrix;

    m_program->setUniformValue(m_modelMatrixLocation, m_modelMatrix);
    m_program->setUniformValue(m_viewMatrixLocation, m_viewMatrix);
    m_program->setUniformValue(m_projectionMatrixLocation, m_projectionMatrix);

//    qDebug() << QTime::currentTime();

    f->glDrawArrays(GL_TRIANGLES, 0, m_triangleData.size() / 3);
}

void GLWindow::keyPressEvent(QKeyEvent *ev) {
    qDebug() << "Got key press event " << ev->key();
    m_currentInputState.m_keysPressed.insert(static_cast<Qt::Key>(ev->key()), true);
}

void GLWindow::keyReleaseEvent(QKeyEvent *ev) {
    qDebug() << "Got key release event " << ev->key();
    m_currentInputState.m_keysPressed.insert(static_cast<Qt::Key>(ev->key()), false);
}

void GLWindow::mouseDoubleClickEvent(QMouseEvent *ev) {
    Q_UNUSED(ev);
}

void GLWindow::mousePressEvent(QMouseEvent *ev) {
    qDebug() << "Got mouse press event " << ev->button();
    m_currentInputState.m_mouseButtonsPressed.insert(ev->button(), true);
}

void GLWindow::mouseMoveEvent(QMouseEvent *ev) {
    qDebug() << "Got mouse move event " << ev->pos();
    m_currentInputState.m_mousePosition = QVector2D(ev->pos().x(), ev->pos().y());
    m_currentInputState.m_mousePositionSet = true;
}

void GLWindow::wheelEvent(QWheelEvent *ev) {
    qDebug() << "Got wheel delta " << ev->angleDelta();
    m_currentInputState.m_scrollAngleDelta += ev->angleDelta().x();
}

void GLWindow::mouseReleaseEvent(QMouseEvent *ev) {
    qDebug() << "Got mouse release event " << ev->button();
    m_currentInputState.m_mouseButtonsPressed.insert(ev->button(), false);
}

void GLWindow::onFrameSwapped() {
    this->update();
}
