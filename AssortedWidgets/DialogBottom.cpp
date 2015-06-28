#include "DialogBottom.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogBottom::DialogBottom(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.m_width=width;
            m_size.m_height=height;
		}

		DialogBottom::~DialogBottom(void)
		{
		}

        void DialogBottom::dragReleased(const Event::MouseEvent &)
		{}

        void DialogBottom::dragMoved(int ,int offsetY)
		{
            Util::Size minimize = m_parent->getPreferedSize();

            if((m_parent->m_size.m_height + offsetY)>minimize.m_height)
			{		
                m_parent->m_size.m_height+=offsetY;
			}
			
            m_parent->pack();
		}
	}
}
