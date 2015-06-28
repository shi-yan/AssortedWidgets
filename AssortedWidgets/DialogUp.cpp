#include "DialogUp.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogUp::DialogUp(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.m_width=width;
            m_size.m_height=height;
		}

		DialogUp::~DialogUp(void)
		{
		}

        void DialogUp::dragReleased(const Event::MouseEvent &)
		{}

        void DialogUp::dragMoved(int ,int offsetY)
		{
            Util::Size minimize = m_parent->getPreferedSize();
			
            if((m_parent->m_size.m_height-offsetY)>minimize.m_height)
			{
                m_parent->m_position.y+=offsetY;
                m_parent->m_size.m_height-=offsetY;
			}
			
            m_parent->pack();
		}

	}
}
