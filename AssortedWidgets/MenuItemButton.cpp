#include "MenuItemButton.h"
#include "MenuBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemButton::MenuItemButton(std::string &_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal)
		{
			size=getPreferedSize();
		}

        MenuItemButton::MenuItemButton(char *_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal)
		{
			size=getPreferedSize();

			MouseDelegate mEntered;
			mEntered.bind(this,&MenuItemButton::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemButton::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemButton::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		MenuItemButton::~MenuItemButton(void)
		{
		}

		void MenuItemButton::mousePressed(const Event::MouseEvent &e)
		{
            m_status=pressed;
        }
		
		void MenuItemButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            m_status=hover;
        }

		void MenuItemButton::mouseReleased(const Event::MouseEvent &e)
		{
            m_status=normal;
			MenuBar::getSingleton().setShrink();
        }

		void MenuItemButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            m_status=normal;
        }
	}
}
