#include "Button.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		Button::Button(std::string &_text):text(_text),AbstractButton(4,4,8,8)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
		}

		Button::Button(char *_text):text(_text),AbstractButton(4,4,8,8)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
		}

		Button::~Button(void)
		{
		}
	}
}