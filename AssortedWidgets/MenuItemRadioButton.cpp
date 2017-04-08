#include "MenuItemRadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemRadioButton::MenuItemRadioButton(const std::string &_text)
            : m_left(24),
              m_right(2),
              m_bottom(2),
              m_top(4),
              m_text(_text),
              m_style(stretch),
              m_status(normal),
              m_toggle(false),
              m_group(nullptr)
		{
            m_size=getPreferedSize();

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuItemRadioButton::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuItemRadioButton::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuItemRadioButton::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuItemRadioButton::mouseReleased));
		}

		MenuItemRadioButton::~MenuItemRadioButton(void)
		{
		}

        void MenuItemRadioButton::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
        }
		
        void MenuItemRadioButton::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            m_status=hover;
        }

        void MenuItemRadioButton::mouseReleased(const Event::MouseEvent &)
		{
            m_status=hover;
            m_group->setSelection(this);
            m_toggle=true;
        }

        void MenuItemRadioButton::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            m_status=normal;
        }
	}
}
