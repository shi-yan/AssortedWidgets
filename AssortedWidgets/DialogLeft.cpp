#include "DialogLeft.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogLeft::DialogLeft(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogLeft::~DialogLeft(void)
		{
		}

		void DialogLeft::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogLeft::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.width-offsetX)>minimize.width)
			{
				parent->position.x+=offsetX;
				parent->size.width-=offsetX;			
			}

			parent->pack();
		}
	}
}