#include "CheckButton.h"


namespace AssortedWidgets
{
	namespace Widgets
	{			
		CheckButton::CheckButton(std::string &_text,bool _check):text(_text),check(_check)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	

			MouseDelegate mReleased;
			mReleased.bind(this,&CheckButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}
		
		CheckButton::CheckButton(char *_text,bool _check):text(_text),check(_check)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			MouseDelegate mReleased;
			mReleased.bind(this,&CheckButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		void CheckButton::mouseReleased(const Event::MouseEvent &e)
		{
			check=!check;
		}

		CheckButton::~CheckButton(void)
		{
		}
	}
}