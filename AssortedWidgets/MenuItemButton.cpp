#include "MenuItemButton.h"
#include "MenuBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		MenuItemButton::MenuItemButton(std::string &_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal)
		{
			size=getPreferedSize();
		}

		MenuItemButton::MenuItemButton(char *_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal)
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
			status=pressed;
		};
		
		void MenuItemButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			status=hover;
		};

		void MenuItemButton::mouseReleased(const Event::MouseEvent &e)
		{
			status=normal;
			MenuBar::getSingleton().setShrink();
		};

		void MenuItemButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			status=normal;
		};
	}
}