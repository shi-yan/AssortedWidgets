#include "Label.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

        Label::Label(const std::string &_text)
            :m_text(_text),
              m_top(4),
              m_bottom(4),
              m_left(10),
              m_right(10),
              m_drawBackground(false)
		{
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;
            m_size=getPreferedSize();
		}

		Label::~Label(void)
		{
		}
	}
}
