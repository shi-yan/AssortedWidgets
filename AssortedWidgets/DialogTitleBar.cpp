#include "DialogTitleBar.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        DialogTitleBar::DialogTitleBar(const std::string &_text)
            :m_text(_text),
              m_left(10),
              m_right(10),
              m_bottom(4),
              m_top(4)
		{
		}

        DialogTitleBar::DialogTitleBar(const char *_text)
            :m_text(_text),
              m_left(10),
              m_right(10),
              m_bottom(4),
              m_top(4)
		{
		}

        DialogTitleBar::~DialogTitleBar(void)
		{
		}

        void DialogTitleBar::dragReleased(const Event::MouseEvent &e)
		{

		}

        void DialogTitleBar::dragMoved(int offsetX,int offsetY)
		{
            m_parent->m_position.x+=offsetX;
            m_parent->m_position.y+=offsetY;
		}
	}
}
