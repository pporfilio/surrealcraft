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
    QVector2D m_mousePosition;

    float m_scrollAngleDelta = 0;
};

#endif // INPUTSTATE_H
