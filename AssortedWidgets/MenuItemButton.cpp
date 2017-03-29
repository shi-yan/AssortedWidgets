#include "MenuItemButton.h"
#include "MenuBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemButton::MenuItemButton(const std::string &_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal)
		{
            m_size=getPreferedSize();
		}

        MenuItemButton::MenuItemButton(const char *_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal)
		{
            m_size=getPreferedSize();

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuItemButton::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuItemButton::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuItemButton::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuItemButton::mouseReleased));
		}

		MenuItemButton::~MenuItemButton(void)
		{
		}

        void MenuItemButton::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
        }
		
        void MenuItemButton::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            m_status=hover;
        }

        void MenuItemButton::mouseReleased(const Event::MouseEvent &)
		{
            m_status=normal;
			MenuBar::getSingleton().setShrink();
        }

        void MenuItemButton::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            m_status=normal;
        }
	}
}
