#include "DialogUpRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogUpRight::DialogUpRight(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.width=width;
            m_size.height=height;
		}

		DialogUpRight::~DialogUpRight(void)
		{
		}
		
		void DialogUpRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogUpRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
            if((parent->m_size.width+offsetX)>minimize.width)
			{
                parent->m_size.width+=offsetX;
			}
			
            if((parent->m_size.height-offsetY)>minimize.height)
			{
                parent->m_position.y+=offsetY;
                parent->m_size.height-=offsetY;
			}
			
			parent->pack();
		}
	}
}
