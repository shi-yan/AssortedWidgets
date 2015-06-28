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
                int tempX=origin.x+m_left;
                int tempY=origin.y+m_top;
				unsigned nextY=0;
                unsigned int width=area.width-m_left;
                unsigned int height=area.height-m_top-m_bottom;

				Util::Size preferedSize=componentList[0]->getPreferedSize();
                componentList[0]->m_position.x=tempX;
                componentList[0]->m_position.y=tempY;
                tempX+=preferedSize.width+m_spacer;
				nextY=std::max<unsigned int>(nextY,preferedSize.height);
				
				for(size_t i=1;i<componentList.size();++i)
				{
					preferedSize=componentList[i]->getPreferedSize();
					if((tempX+preferedSize.width)>width)
					{
                        tempX=origin.x+m_left;
                        tempY+=nextY+m_spacer;
						nextY=0;
                        componentList[i]->m_position.x=tempX;
                        componentList[i]->m_position.y=tempY;
                        tempX+=preferedSize.width+m_spacer;
						nextY=std::max<unsigned int>(nextY,preferedSize.height);
					}
					else
					{
                        componentList[i]->m_position.x=tempX;
                        componentList[i]->m_position.y=tempY;
                        tempX+=preferedSize.width+m_spacer;
						nextY=std::max<unsigned int>(nextY,preferedSize.height);
					}
				}
			}
		};

        Util::Size FlowLayout::getPreferedSize() const
		{
            return Util::Size(10+m_left+m_right,10+m_top+m_bottom);
        }
	}
}
