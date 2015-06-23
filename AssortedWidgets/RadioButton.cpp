#include "RadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		RadioButton::RadioButton(std::string &_text,RadioGroup *_group):text(_text),group(_group),check(false)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	
			MouseDelegate mReleased;
			mReleased.bind(this,&RadioButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		RadioButton::RadioButton(char *_text,RadioGroup *_group):text(_text),group(_group),check(false)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;	
			MouseDelegate mReleased;
			mReleased.bind(this,&RadioButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
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