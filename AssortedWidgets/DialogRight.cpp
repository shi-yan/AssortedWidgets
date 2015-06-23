#include "DialogRight.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogRight::DialogRight(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogRight::~DialogRight(void)
		{
		}

		void DialogRight::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogRight::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.width+offsetX)>minimize.width)
			{
				parent->size.width+=offsetX;			
			}

			parent->pack();
		}
	}
}