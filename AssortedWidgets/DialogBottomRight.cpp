#include "DialogBottomRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogBottomRight::DialogBottomRight(int x,int y,unsigned int width,unsigned int height)
		{
            m_position.x=x;
            m_position.y=y;
            m_size.width=width;
            m_size.height=height;
		}

		DialogBottomRight::~DialogBottomRight(void)
		{
		}
		
		void DialogBottomRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogBottomRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
            if((parent->m_size.width+offsetX)>minimize.width)
			{
                parent->m_size.width+=offsetX;
			}
			
            if((parent->m_size.height+offsetY)>minimize.height)
			{
				
                parent->m_size.height+=offsetY;
			}
			
			parent->pack();
		}
	}
}
