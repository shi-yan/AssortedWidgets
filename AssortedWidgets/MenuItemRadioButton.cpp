#include "MenuItemRadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		MenuItemRadioButton::MenuItemRadioButton(std::string &_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal),toggle(false)
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

		MenuItemRadioButton::MenuItemRadioButton(char *_text):text(_text),style(stretch),left(24),top(4),bottom(2),right(2),status(normal),toggle(false)
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
			status=pressed;
		};
		
		void MenuItemRadioButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			status=hover;
		};

		void MenuItemRadioButton::mouseReleased(const Event::MouseEvent &e)
		{
			status=hover;
			group->setSelection(this);
			toggle=true;
		};

		void MenuItemRadioButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			status=normal;
		};
	}
}