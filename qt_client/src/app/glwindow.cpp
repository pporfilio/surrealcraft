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
#include <stdint.h>


GLWindow::GLWindow() : m_voxelData(VectorI3D(10, 20, 1))
{
    resize(800, 600);

    m_timer.start();
    m_previousFrameTime = m_timer.msecsSinceReference();

    QObject::connect(this, SIGNAL(frameSwapped()), this, SLOT(onFrameSwapped()));

    bool present = true;
    for (int x = 0; x < 10; ++x) {
        for (int y = 0; y < 20; ++y) {
            if (present) {
                m_voxelData.setDataAt(VectorI3D(x, y, 0), Voxel(1, 0.5, 0.001 * x * y, 0.2));
            }
            present = !present;
            qDebug() << present;
        }
    }


    for (int x = 0; x < m_voxelData.dimensions().x(); ++x) {
        for (int y = 0; y < m_voxelData.dimensions().y(); ++y) {
            for (int z = 0; z < m_voxelData.dimensions().z(); ++z) {
                qDebug() << x << y << z;
                Voxel voxel = m_voxelData.dataAt(VectorI3D(x, y, z));
                qDebug() << voxel.m_value << voxel.m_g;
                if (voxel.m_value != 0) {
                    addCube(QVector3D(x + 0.5, y + 0.5, z + 0.5), QVector3D(voxel.m_r, voxel.m_g, voxel.m_b));
                }
            }
        }
    }
//    addCube(QVector3D(5, 0, 0), QVector3D(1, 0, 0));
//    addCube(QVector3D(-5, 0, 0), QVector3D(1, 0, 0));
//    addCube(QVector3D(0, 5, 0), QVector3D(0, 1, 0));
//    addCube(QVector3D(0, -5, 0), QVector3D(0, 1, 0));
//    addCube(QVector3D(0, 0, 5), QVector3D(0, 0, 1));
//    addCube(QVector3D(0, 0, -5), QVector3D(0, 0, 1));
}

GLWindow::~GLWindow() {
    makeCurrent();
}

void GLWindow::addCube(QVector3D center, QVector3D color) {
    this->addVertex(QVector3D(-0.f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f), center, color);

    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f), center, color);

    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f), center, color);

    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);

    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f, -0.5f, -0.5f), center, color);

    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f, -0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f,  0.5f), center, color);
    this->addVertex(QVector3D(-0.5f,  0.5f, -0.5f), center, color);

}

void GLWindow::addVertex(QVector3D v, QVector3D translation, QVector3D color) {
    m_triangleData.append(v.x() + translation.x());
    m_triangleData.append(v.y() + translation.y());
    m_triangleData.append(v.z() + translation.z());
    m_triangleData.append(color.x());
    m_triangleData.append(color.y());
    m_triangleData.append(color.z());
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

    // This says that layout(location = 0) starts at 0 (nullptr) offset into the VBO
    f->glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 6 * sizeof(GLfloat), nullptr);

    // This enables `layout(location = 0) in vec4 vertex;` from the shader
    f->glEnableVertexAttribArray(0);

    f->glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 6 * sizeof(GLfloat), reinterpret_cast<void *>(3 * sizeof(GLfloat)));
    f->glEnableVertexAttribArray(1);


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
    return (inputState.m_mouseDelta.x() - previousInputState.m_mouseDelta.x()) * rotateScale;
}

float GLWindow::getCameraPitchDeltaDeg(const InputState &inputState,
                                       const InputState &previousInputState) {
    float rotateScale = 0.1;
    if (!previousInputState.m_mousePositionSet) {
        return 0;
    }
    // Negative one because y is bigger at the bottom than the top of the window.
    return -1 * (inputState.m_mouseDelta.y() - previousInputState.m_mouseDelta.y()) * rotateScale;
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

    m_program->setUniformValue(m_modelMatrixLocation, m_modelMatrix);
    m_program->setUniformValue(m_viewMatrixLocation, m_viewMatrix);
    m_program->setUniformValue(m_projectionMatrixLocation, m_projectionMatrix);

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
    int deltaX = ev->pos().x() - width() / 2;
    int deltaY = ev->pos().y() - height() / 2;
    if (deltaX == 0 && deltaY == 0) {
        // If the mouse is at the center, then this should be because we moved the cursor
        // there after reading the user's last move.
        return;
    }

    m_currentInputState.m_mouseDelta += QVector2D(deltaX, deltaY);
    m_currentInputState.m_mousePositionSet = true;
    QCursor::setPos(mapToGlobal(QPoint(width() / 2, height() / 2)));
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
