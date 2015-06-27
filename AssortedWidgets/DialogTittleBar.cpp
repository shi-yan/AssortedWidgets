#include "DialogTittleBar.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        DialogTittleBar::DialogTittleBar(std::string &_text)
            :m_text(_text),
              m_left(10),
              m_right(10),
              m_bottom(4),
              m_top(4)
		{
		}

        DialogTittleBar::DialogTittleBar(char *_text)
            :m_text(_text),
              m_left(10),
              m_right(10),
              m_bottom(4),
              m_top(4)
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
            m_parent->position.x+=offsetX;
            m_parent->position.y+=offsetY;
		}
	}
}
