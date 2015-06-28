#include "DialogRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogRight::DialogRight(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.width=width;
            m_size.height=height;
		}

		DialogRight::~DialogRight(void)
		{
		}

		void DialogRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
            if((parent->m_size.width+offsetX)>minimize.width)
			{
                parent->m_size.width+=offsetX;
			}

			parent->pack();
		}
	}
}
