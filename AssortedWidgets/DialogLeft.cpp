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
            m_size.m_width=width;
            m_size.m_height=height;
		}

		DialogLeft::~DialogLeft(void)
		{
		}

        void DialogLeft::dragReleased(const Event::MouseEvent &)
		{}

        void DialogLeft::dragMoved(int offsetX, int )
		{
            Util::Size minimize = m_parent->getPreferedSize();
			
            if((m_parent->m_size.m_width-offsetX)>minimize.m_width)
			{
                m_parent->m_position.x+=offsetX;
                m_parent->m_size.m_width-=offsetX;
			}

            m_parent->pack();
		}
	}
}
