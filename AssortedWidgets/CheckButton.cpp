#include "CheckButton.h"


namespace AssortedWidgets
{
	namespace Widgets
	{			
		CheckButton::CheckButton(std::string &_text,bool _check):text(_text),check(_check)
		{
            m_size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	

            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(CheckButton::mouseReleased));
		}
		
		CheckButton::CheckButton(char *_text,bool _check):text(_text),check(_check)
		{
            m_size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(CheckButton::mouseReleased));
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
