#include "ScrollBarButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		ScrollBarButton::ScrollBarButton(int _type):type(_type),AbstractButton(0,0,0,0)
		{
			size.width=15;
			size.height=15;
		}

		ScrollBarButton::~ScrollBarButton(void)
		{
		}
	}
}