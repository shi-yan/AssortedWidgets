#include "DropListButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		DropListButton::DropListButton(void)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
		}

		DropListButton::~DropListButton(void)
		{
		}
	}
}