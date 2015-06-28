#include "ScrollBarButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		ScrollBarButton::ScrollBarButton(int _type):type(_type),AbstractButton(0,0,0,0)
		{
            m_size.width=15;
            m_size.height=15;
		}

		ScrollBarButton::~ScrollBarButton(void)
		{
		}
	}
}
