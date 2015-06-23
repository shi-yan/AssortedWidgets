#include "DialogBottomRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogBottomRight::DialogBottomRight(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogBottomRight::~DialogBottomRight(void)
		{
		}
		
		void DialogBottomRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogBottomRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.width+offsetX)>minimize.width)
			{
				parent->size.width+=offsetX;			
			}
			
			if((parent->size.height+offsetY)>minimize.height)
			{
				
				parent->size.height+=offsetY;
			}
			
			parent->pack();
		}
	}
}