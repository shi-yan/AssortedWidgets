#include "DialogUp.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DialogUp::DialogUp(int x,int y,unsigned int width,unsigned int height)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
		}

		DialogUp::~DialogUp(void)
		{
		}

		void DialogUp::dragReleased(const Event::MouseEvent &e)
		{}

		void DialogUp::dragMoved(int offsetX,int offsetY)
		{
			Util::Size minimize=parent->getPreferedSize();
			
			if((parent->size.height-offsetY)>minimize.height)
			{
				parent->position.y+=offsetY;
				parent->size.height-=offsetY;
			}
			
			parent->pack();
		}

	}
}