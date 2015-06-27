#include "MenuItemToggleButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemToggleButton::MenuItemToggleButton(std::string &_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal),
              m_toggle(false)
		{
			size=getPreferedSize();
		}

        MenuItemToggleButton::MenuItemToggleButton(char *_text)
            :m_text(_text),
              m_style(stretch),
              m_left(24),
              m_top(4),
              m_bottom(2),
              m_right(2),
              m_status(normal),
              m_toggle(false)
		{
			size=getPreferedSize();

			MouseDelegate mEntered;
			mEntered.bind(this,&MenuItemToggleButton::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemToggleButton::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemToggleButton::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemToggleButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		MenuItemToggleButton::~MenuItemToggleButton(void)
		{
		}

		void MenuItemToggleButton::mousePressed(const Event::MouseEvent &e)
		{
            m_status=pressed;
        }
		
		void MenuItemToggleButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            m_status=hover;
        }

		void MenuItemToggleButton::mouseReleased(const Event::MouseEvent &e)
		{
            m_status=hover;
            m_toggle=!m_toggle;
        }

		void MenuItemToggleButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            m_status=normal;
        }
	}
}
