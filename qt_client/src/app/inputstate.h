#ifndef INPUTSTATE_H
#define INPUTSTATE_H

#include <QMap>
#include <QVector2D>

class InputState
{
public:
    InputState();

    QMap<Qt::Key, bool> m_keysPressed;
    QMap<Qt::MouseButton, bool> m_mouseButtonsPressed;

    bool m_mousePositionSet = false;
    QVector2D m_mouseDelta;

    float m_scrollAngleDelta = 0;

    bool keyPressed(Qt::Key key) const;
};

#endif // INPUTSTATE_H
