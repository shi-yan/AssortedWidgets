#include "AbstractButton.h"
#include <functional>

namespace AssortedWidgets
{
	namespace Widgets
	{
        AbstractButton::AbstractButton(unsigned int top, unsigned int bottom, unsigned int left, unsigned int right, enum Status status)
            :m_top(top),
              m_bottom(bottom),
              m_left(left),
              m_right(right),
              m_status(status)
		{
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(AbstractButton::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(AbstractButton::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(AbstractButton::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(AbstractButton::mouseReleased));
		}

        void AbstractButton::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
        }
		
        void AbstractButton::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            m_status=hover;
        }

        void AbstractButton::mouseReleased(const Event::MouseEvent &)
		{
            m_status=hover;
        }

        void AbstractButton::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            m_status=normal;
        }

		AbstractButton::~AbstractButton(void)
		{
		}
	}
}
