#include "DropListButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		DropListButton::DropListButton(void)
		{
            m_size=getPreferedSize();
            m_horizontalStyle=Element::Fit;
            m_verticalStyle=Element::Fit;
		}

		DropListButton::~DropListButton(void)
		{
		}
	}
}
