#include "ScrollBarButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        ScrollBarButton::ScrollBarButton(int _type)
            :m_type(_type),
              AbstractButton(0,0,0,0)
		{
            m_size.m_width=15;
            m_size.m_height=15;
		}

		ScrollBarButton::~ScrollBarButton(void)
		{
		}
	}
}
