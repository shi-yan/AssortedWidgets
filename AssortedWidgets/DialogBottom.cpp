#include "DialogBottom.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogBottom::DialogBottom(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogBottom::~DialogBottom(void)
		{
		}

		void DialogBottom::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogBottom::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();

			if((parent->size.height+offsetY)>minimize.height)
			{
			
				parent->size.height+=offsetY;
			}
			
			parent->pack();
		}
	}
}