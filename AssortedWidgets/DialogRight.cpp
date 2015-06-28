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
            m_size.m_width=width;
            m_size.m_height=height;
		}

		DialogRight::~DialogRight(void)
		{
		}

		void DialogRight::dragReleased(const Event::MouseEvent &e)
		{}

        void DialogRight::dragMoved(int offsetX,int )
		{
            Util::Size minimize=m_parent->getPreferedSize();
			
            if((m_parent->m_size.m_width+offsetX)>minimize.m_width)
			{
                m_parent->m_size.m_width+=offsetX;
			}

            m_parent->pack();
		}
	}
}
