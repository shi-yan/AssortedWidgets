#include "Label.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		Label::Label(std::string &_text):text(_text),top(4),right(10),left(10),bottom(4),drawBackground(false)
		{
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			size=getPreferedSize();
		}

		Label::Label(char *_text):text(_text),top(4),right(6),left(6),bottom(4),drawBackground(false)
		{
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			size=getPreferedSize();
		}

		Label::~Label(void)
		{
		}
	}
}