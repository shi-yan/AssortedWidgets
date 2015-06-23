#include "DialogBottomLeft.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogBottomLeft::DialogBottomLeft(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogBottomLeft::~DialogBottomLeft(void)
		{
		}
		
		void DialogBottomLeft::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogBottomLeft::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.width-offsetX)>minimize.width)
			{
				parent->position.x+=offsetX;
				parent->size.width-=offsetX;			
			}
			
			if((parent->size.height+offsetY)>minimize.height)
			{
				parent->size.height+=offsetY;
			}
			
			parent->pack();
		}
	}
}