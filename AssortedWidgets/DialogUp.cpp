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
            m_size.width=width;
            m_size.height=height;
		}

		DialogUp::~DialogUp(void)
		{
		}

		void DialogUp::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogUp::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
            if((parent->m_size.height-offsetY)>minimize.height)
			{
                parent->m_position.y+=offsetY;
                parent->m_size.height-=offsetY;
			}
			
			parent->pack();
		}

	}
}
