#include "DialogLeft.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogLeft::DialogLeft(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.width=width;
            m_size.height=height;
		}

		DialogLeft::~DialogLeft(void)
		{
		}

		void DialogLeft::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogLeft::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
            if((parent->m_size.width-offsetX)>minimize.width)
			{
                parent->m_position.x+=offsetX;
                parent->m_size.width-=offsetX;
			}

			parent->pack();
		}
	}
}
