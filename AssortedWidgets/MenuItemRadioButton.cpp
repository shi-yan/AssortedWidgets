#include "MenuItemRadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemRadioButton::MenuItemRadioButton(std::string &_text)
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
			mEntered.bind(this,&MenuItemRadioButton::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemRadioButton::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemRadioButton::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemRadioButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

        MenuItemRadioButton::MenuItemRadioButton(char *_text)
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
			mEntered.bind(this,&MenuItemRadioButton::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemRadioButton::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemRadioButton::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemRadioButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		MenuItemRadioButton::~MenuItemRadioButton(void)
		{
		}

		void MenuItemRadioButton::mousePressed(const Event::MouseEvent &e)
		{
            m_status=pressed;
        }
		
		void MenuItemRadioButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            m_status=hover;
        }

		void MenuItemRadioButton::mouseReleased(const Event::MouseEvent &e)
		{
            m_status=hover;
            m_group->setSelection(this);
            m_toggle=true;
        }

		void MenuItemRadioButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            m_status=normal;
        }
	}
}
