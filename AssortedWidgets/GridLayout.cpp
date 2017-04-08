#include "GridLayout.h"
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Layout
	{
        GridLayout::~GridLayout(void)
		{
            for (size_t i = 0; i < m_rowCount; ++i)
                delete [] m_alignment[i];
            delete [] m_alignment;
		}

        void GridLayout::updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area)
		{
			std::vector<Widgets::Element*>::iterator iter(componentList.begin());
            for(size_t i=0;i<m_rowCount;++i)
			{
                for(size_t e=0;e<m_columnCount;++e)
				{
					if(iter<componentList.end())
					{
                        m_alignment[i][e].m_component=(*iter);
						Util::Size perfectSize=(*iter)->getPreferedSize();
                        m_alignment[i][e].m_width=perfectSize.m_width;
                        m_alignment[i][e].m_height=perfectSize.m_height;
                        m_alignment[i][e].m_HStyle=(*iter)->getHorizontalStyle();
                        m_alignment[i][e].m_VStyle=(*iter)->getVerticalStyle();
						++iter;
					}
					else
					{
                        m_alignment[i][e].m_component=0;
                        m_alignment[i][e].m_width=0;
                        m_alignment[i][e].m_height=0;
                        m_alignment[i][e].m_HStyle=Widgets::Element::Fit;
                        m_alignment[i][e].m_VStyle=Widgets::Element::Fit;
					}
				}
			}

			struct OneLineInfo
			{
                unsigned int miniSize;
                bool isStretch;
			};

            struct OneLineInfo *columnInfo=new struct OneLineInfo[m_columnCount];

            for(size_t e=0;e<m_columnCount;++e)
			{
                columnInfo[e].miniSize=0;
				columnInfo[e].isStretch=false;
                for(size_t i=0;i<m_rowCount;++i)
				{
                    if(m_alignment[i][e].m_HStyle==Widgets::Element::Stretch)
					{
						columnInfo[e].isStretch=true;
					}
                    columnInfo[e].miniSize=std::max<unsigned int>(columnInfo[e].miniSize,m_alignment[i][e].m_width);
				}
			}

            struct OneLineInfo *rowInfo=new struct OneLineInfo[m_rowCount];

            for(size_t i=0;i<m_rowCount;++i)
			{
				rowInfo[i].miniSize=0;
				rowInfo[i].isStretch=false;
                for(size_t e=0;e<m_columnCount;++e)
				{
                    if(m_alignment[i][e].m_VStyle==Widgets::Element::Stretch)
					{
						rowInfo[i].isStretch=true;
					}
                    rowInfo[i].miniSize=std::max<unsigned int>(rowInfo[i].miniSize,m_alignment[i][e].m_height);
				}
			}

            int widthAvailable(area.m_width-(m_columnCount-1)*m_spacer-m_left-m_right);
			unsigned int stretchSegment(0);
            for(size_t e=0;e<m_columnCount;++e)
			{
				if(columnInfo[e].isStretch)
				{
					++stretchSegment;
				}
				else
				{
					widthAvailable-=columnInfo[e].miniSize;
				}
			}
				
			if(widthAvailable>0)
			{
				if(stretchSegment)
				{
					unsigned int averageWidth(widthAvailable/stretchSegment);
                    for(size_t e=0;e<m_columnCount;++e)
					{
						if(columnInfo[e].isStretch)
						{
							columnInfo[e].miniSize=std::max<unsigned int>(columnInfo[e].miniSize,averageWidth);
						}
					}
				}
				else
				{
                    unsigned int averageAppend(widthAvailable/m_columnCount);
                    for(size_t e=0;e<m_columnCount;++e)
					{
						columnInfo[e].miniSize+=averageAppend;
					}
				}
			}

            int heightAvailable(area.m_height-m_top-m_bottom-(m_rowCount-1)*m_spacer);
			stretchSegment=0;
            for(size_t i=0;i<m_rowCount;++i)
			{
				if(rowInfo[i].isStretch)
				{
					++stretchSegment;
				}
				else
				{
					heightAvailable-=rowInfo[i].miniSize;
				}
			}

			if(heightAvailable>0)
			{
				if(stretchSegment)
				{
					unsigned int averageHeight(heightAvailable/stretchSegment);
                    for(size_t i=0;i<m_rowCount;++i)
					{
						if(rowInfo[i].isStretch)
						{
							rowInfo[i].miniSize=std::max<unsigned int>(rowInfo[i].miniSize,averageHeight);
						}
					}
				}
				else
				{
                    unsigned int averageAppend(heightAvailable/m_rowCount);
                    for(size_t i=0;i<m_rowCount;++i)
					{
						rowInfo[i].miniSize+=averageAppend;
					}
				}
			}

            int tempX=m_left+origin.x;
            int tempY=m_top+origin.y;

            for(size_t i=0;i<m_rowCount;++i)
			{
                for(size_t e=0;e<m_columnCount;++e)
				{
					Util::Position Cposition(tempX,tempY);
					Util::Size Carea(columnInfo[e].miniSize,rowInfo[i].miniSize);
					orderComponent(static_cast<unsigned int>(i),static_cast<unsigned int>(e),Cposition,Carea);
                    tempX+=columnInfo[e].miniSize+m_spacer;
				}
                tempX=m_left+origin.x;
                tempY+=m_spacer+rowInfo[i].miniSize;
			}

			delete [] columnInfo;
			delete [] rowInfo;
		}

        void GridLayout::orderComponent(unsigned int row,unsigned int column,Util::Position &origin,Util::Size &area)
		{
            struct Alignment component=m_alignment[row][column];
            if(component.m_component)
			{
                if(component.m_HStyle==Widgets::Element::Stretch)
				{
                    component.m_component->m_size.m_width=area.m_width;
                    component.m_component->m_position.x=origin.x;
				}
				else
				{
                    switch(component.m_HAlignment)
					{
						case HLeft:
						{
                            component.m_component->m_position.x=origin.x;
							break;
						}
						case HCenter:
						{
                            component.m_component->m_position.x=static_cast<int>(origin.x+(area.m_width-component.m_width)*0.5f);
							break;
						}
						case HRight:
						{
                            component.m_component->m_position.x=origin.x+(area.m_width-component.m_width);
							break;
						}
					}
				}

                if(component.m_VStyle==Widgets::Element::Stretch)
				{
                    component.m_component->m_size.m_height=area.m_height;
                    component.m_component->m_position.y=origin.y;
				}
				else
				{
                    switch(component.m_VAlignment)
					{
						case VTop:
						{
                            component.m_component->m_position.y=origin.y;
							break;
						}
						case VCenter:
						{
                            component.m_component->m_position.y=static_cast<int>(origin.y+(area.m_height-component.m_height)*0.5f);
							break;
						}
						case VBottom:
						{
                            component.m_component->m_position.y=origin.y+(area.m_height-component.m_height);
							break;
						}
					}
				}

                component.m_component->pack();
			}
		}

        Util::Size GridLayout::getPreferedSize() const
		{
			return Util::Size();
		}
	}
}
