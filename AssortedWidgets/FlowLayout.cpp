#include "FlowLayout.h"
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Layout
	{
		FlowLayout::~FlowLayout(void)
		{
		}

		void FlowLayout::updateLayout(std::vector<Widgets::Element*> &componentList,Util::Position &origin,Util::Size &area)
		{
			if(!componentList.empty())
			{
				int tempX=origin.x+left;
				int tempY=origin.y+top;
				unsigned nextY=0;
				unsigned int width=area.width-left;
				unsigned int height=area.height-top-bottom;

				Util::Size preferedSize=componentList[0]->getPreferedSize();
				componentList[0]->position.x=tempX;
				componentList[0]->position.y=tempY;
				tempX+=preferedSize.width+spacer;
				nextY=std::max<unsigned int>(nextY,preferedSize.height);
				
				for(size_t i=1;i<componentList.size();++i)
				{
					preferedSize=componentList[i]->getPreferedSize();
					if((tempX+preferedSize.width)>width)
					{
						tempX=origin.x+left;
						tempY+=nextY+spacer;
						nextY=0;
						componentList[i]->position.x=tempX;
						componentList[i]->position.y=tempY;
						tempX+=preferedSize.width+spacer;
						nextY=std::max<unsigned int>(nextY,preferedSize.height);
					}
					else
					{
						componentList[i]->position.x=tempX;
						componentList[i]->position.y=tempY;
						tempX+=preferedSize.width+spacer;
						nextY=std::max<unsigned int>(nextY,preferedSize.height);
					}
				}
			}
		};

		Util::Size FlowLayout::getPreferedSize()
		{
			return Util::Size(10+left+right,10+top+bottom);
		};
	}
}