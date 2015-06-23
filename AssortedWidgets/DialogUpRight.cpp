#include "DialogUpRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogUpRight::DialogUpRight(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogUpRight::~DialogUpRight(void)
		{
		}
		
		void DialogUpRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogUpRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.width+offsetX)>minimize.width)
			{
				parent->size.width+=offsetX;			
			}
			
			if((parent->size.height-offsetY)>minimize.height)
			{
				parent->position.y+=offsetY;
				parent->size.height-=offsetY;
			}
			
			parent->pack();
		}
	}
}