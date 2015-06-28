#include "RadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		RadioButton::RadioButton(std::string &_text,RadioGroup *_group):text(_text),group(_group),check(false)
		{
            m_size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	

            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(RadioButton::mouseReleased));
		}

		RadioButton::RadioButton(char *_text,RadioGroup *_group):text(_text),group(_group),check(false)
		{
            m_size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	

            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(RadioButton::mouseReleased));
		}

		void RadioButton::mouseReleased(const Event::MouseEvent &e)
		{
			if(!check)
			{
				group->setCheck(this);
				//check=true;
			}
		}

		RadioButton::~RadioButton(void)
		{
		}
	}
}
