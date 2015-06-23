#include "MenuItemToggleButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		MenuItemToggleButton::MenuItemToggleButton(std::string &_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal),toggle(false)
		{
			size=getPreferedSize();
		}

		MenuItemToggleButton::MenuItemToggleButton(char *_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal),toggle(false)
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
			status=pressed;
		};
		
		void MenuItemToggleButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			status=hover;
		};

		void MenuItemToggleButton::mouseReleased(const Event::MouseEvent &e)
		{
			status=hover;
			toggle=!toggle;
		};

		void MenuItemToggleButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			status=normal;
		};
	}
}