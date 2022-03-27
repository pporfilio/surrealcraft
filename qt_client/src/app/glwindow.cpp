#include "glwindow.h"

GLWindow::GLWindow()
{
    m_uniformMatrices.insert({m_WORLD_MATRIX_NAME, QMatrix4x4()});
}

GLWindow::~GLWindow() {
    makeCurrent();
}

void GLWindow::initializeGL() {
    QOpenGLFunctions *f = QOpenGLContext::currentContext()->functions();

    // TODO load texture(s)

    // TODO load program(s)

    m_uniformLocations.insert({m_PROJECTION_MATRIX_NAME, m_program->uniformLocation(m_PROJECTION_MATRIX_NAME.c_str())});


}

void GLWindow::resizeGL(int w, int h) {

}

void GLWindow::paintGL() {

}
