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
            m_size.width=width;
            m_size.height=height;
		}

		DialogBottom::~DialogBottom(void)
		{
		}

		void DialogBottom::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogBottom::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();

            if((parent->m_size.height+offsetY)>minimize.height)
			{
			
                parent->m_size.height+=offsetY;
			}
			
			parent->pack();
		}
	}
}
