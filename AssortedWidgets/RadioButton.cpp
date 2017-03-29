#include "RadioButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

        RadioButton::RadioButton(const std::string &_text,RadioGroup *_group)
            :m_text(_text),
              m_group(_group),
              m_check(false)
		{
            m_size=getPreferedSize();
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;

            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(RadioButton::mouseReleased));
		}

        RadioButton::RadioButton(const char *_text,RadioGroup *_group)
            :m_text(_text),
              m_group(_group),
              m_check(false)
		{
            m_size=getPreferedSize();
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;

            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(RadioButton::mouseReleased));
		}

        void RadioButton::mouseReleased(const Event::MouseEvent &)
		{
            if(!m_check)
			{
                m_group->setCheck(this);
				//check=true;
			}
		}

		RadioButton::~RadioButton(void)
		{
		}
	}
}
