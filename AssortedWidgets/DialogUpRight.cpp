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
            m_size.m_width=width;
            m_size.m_height=height;
		}

		DialogUpRight::~DialogUpRight(void)
		{
		}
		
		void DialogUpRight::dragReleased(const Event::MouseEvent &e)
        {
            (void) e;
        }

		void DialogUpRight::dragMoved(int offsetX,int offsetY)
		{
            Util::Size minimize=m_parent->getPreferedSize();
			
            if((m_parent->m_size.m_width+offsetX)>minimize.m_width)
			{
                m_parent->m_size.m_width+=offsetX;
			}
			
            if((m_parent->m_size.m_height-offsetY)>minimize.m_height)
			{
                m_parent->m_position.y+=offsetY;
                m_parent->m_size.m_height-=offsetY;
			}
			
            m_parent->pack();
		}
	}
}
