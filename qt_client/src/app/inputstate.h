#ifndef INPUTSTATE_H
#define INPUTSTATE_H

#include <QMap>
#include <QVector2D>

class InputState
{
public:
    InputState();

private:
    QMap<Qt::Key, bool> m_keysPressed;
    QMap<Qt::MouseButton, bool> m_mouseButtonsPressed;
    QVector2D m_mousePosition;
    float m_scrollAngleDelta;
};

#endif // INPUTSTATE_H
