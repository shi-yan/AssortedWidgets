#include "MenuItemToggleButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemToggleButton::MenuItemToggleButton(const std::string &_text)
            : m_left(24),
              m_right(2),
              m_bottom(2),
              m_top(4),
              m_text(_text),
              m_style(stretch),
              m_status(normal),
              m_toggle(false)
		{
            m_size=getPreferedSize();
		}

		MenuItemToggleButton::~MenuItemToggleButton(void)
		{
		}

        void MenuItemToggleButton::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
        }
		
        void MenuItemToggleButton::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            m_status=hover;
        }

        void MenuItemToggleButton::mouseReleased(const Event::MouseEvent &)
		{
            m_status=hover;
            m_toggle=!m_toggle;
        }

        void MenuItemToggleButton::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            m_status=normal;
        }
	}
}
