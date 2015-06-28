#include "RadioGroup.h"
#include "RadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        RadioGroup::RadioGroup(void)
            :m_currentChecked(0)
		{
		}

		void RadioGroup::setCheck(RadioButton *_currentChecked)
		{
            if(m_currentChecked)
			{
                m_currentChecked->checkOff();
			}
            m_currentChecked=_currentChecked;
            m_currentChecked->checkOn();
		}

		RadioGroup::~RadioGroup(void)
		{
		}
	}
}
