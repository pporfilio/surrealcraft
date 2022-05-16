#include "inputstate.h"

InputState::InputState()
{
}

bool InputState::keyPressed(Qt::Key key) const {
    return m_keysPressed.value(key, false);
}
