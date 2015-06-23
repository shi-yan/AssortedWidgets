#include "RadioGroup.h"
#include "RadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		RadioGroup::RadioGroup(void):currentChecked(0)
		{
		}

		void RadioGroup::setCheck(RadioButton *_currentChecked)
		{
			if(currentChecked)
			{
				currentChecked->checkOff();
			}
			currentChecked=_currentChecked;
			currentChecked->checkOn();
		}

		RadioGroup::~RadioGroup(void)
		{
		}
	}
}