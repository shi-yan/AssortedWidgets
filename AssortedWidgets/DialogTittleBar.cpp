#include "DialogTittleBar.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogTittleBar::DialogTittleBar(std::string &_text):text(_text),left(10),right(10),bottom(4),top(4)
		{
		}

		DialogTittleBar::DialogTittleBar(char *_text):text(_text),left(10),right(10),bottom(4),top(4)
		{
		}

		DialogTittleBar::~DialogTittleBar(void)
		{
		}

		void DialogTittleBar::dragReleased(const Event::MouseEvent &e)
		{

		}

		void DialogTittleBar::dragMoved(int offsetX,int offsetY)
		{
			parent->position.x+=offsetX;
			parent->position.y+=offsetY;
		}
	}
}