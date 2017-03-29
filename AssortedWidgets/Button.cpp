#include "Button.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        Button::Button(const std::string &text):m_text(text),AbstractButton(4,4,8,8)
		{
            m_size=getPreferedSize();
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;
		}

        Button::Button(const char *text):m_text(text),AbstractButton(4,4,8,8)
		{
            m_size=getPreferedSize();
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;
		}

		Button::~Button(void)
		{
		}
	}
}
