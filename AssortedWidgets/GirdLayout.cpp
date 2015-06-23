#include "GirdLayout.h"
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Layout
	{
		GirdLayout::~GirdLayout(void)
		{
			for (size_t i = 0; i < rowCount; ++i)
				delete [] alignment[i];
			delete [] alignment;
		}

		void GirdLayout::updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area)
		{
			std::vector<Widgets::Element*>::iterator iter(componentList.begin());
			for(size_t i=0;i<rowCount;++i)
			{
				for(size_t e=0;e<columnCount;++e)
				{
					if(iter<componentList.end())
					{
						alignment[i][e].component=(*iter);
						Util::Size perfectSize=(*iter)->getPreferedSize();
						alignment[i][e].width=perfectSize.width;
						alignment[i][e].height=perfectSize.height;
						alignment[i][e].HStyle=(*iter)->getHorizontalStyle();
						alignment[i][e].VStyle=(*iter)->getVerticalStyle();
						++iter;
					}
					else
					{
						alignment[i][e].component=0;
						alignment[i][e].width=0;
						alignment[i][e].height=0;
						alignment[i][e].HStyle=Widgets::Element::Fit;
						alignment[i][e].VStyle=Widgets::Element::Fit;
					}
				}
			}

			struct OneLineInfo
			{
				unsigned int miniSize;
				bool isStretch;
			};

			struct OneLineInfo *columnInfo=new struct OneLineInfo[columnCount];

			for(size_t e=0;e<columnCount;++e)
			{
				columnInfo[e].miniSize=0;
				columnInfo[e].isStretch=false;
				for(size_t i=0;i<rowCount;++i)
				{
					if(alignment[i][e].HStyle==Widgets::Element::Stretch)
					{
						columnInfo[e].isStretch=true;
					}
					columnInfo[e].miniSize=std::max<unsigned int>(columnInfo[e].miniSize,alignment[i][e].width);
				}
			}

			struct OneLineInfo *rowInfo=new struct OneLineInfo[rowCount];

			for(size_t i=0;i<rowCount;++i)
			{
				rowInfo[i].miniSize=0;
				rowInfo[i].isStretch=false;
				for(size_t e=0;e<columnCount;++e)
				{
					if(alignment[i][e].VStyle==Widgets::Element::Stretch)
					{
						rowInfo[i].isStretch=true;
					}
					rowInfo[i].miniSize=std::max<unsigned int>(rowInfo[i].miniSize,alignment[i][e].height);
				}
			}

			int widthAvailable(area.width-(columnCount-1)*spacer-left-right);
			unsigned int stretchSegment(0);
			for(size_t e=0;e<columnCount;++e)
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
					for(size_t e=0;e<columnCount;++e)
					{
						if(columnInfo[e].isStretch)
						{
							columnInfo[e].miniSize=std::max<unsigned int>(columnInfo[e].miniSize,averageWidth);
						}
					}
				}
				else
				{
					unsigned int averageAppend(widthAvailable/columnCount);
					for(size_t e=0;e<columnCount;++e)
					{
						columnInfo[e].miniSize+=averageAppend;
					}
				}
			}

			int heightAvailable(area.height-top-bottom-(rowCount-1)*spacer);
			stretchSegment=0;
			for(size_t i=0;i<rowCount;++i)
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
					for(size_t i=0;i<rowCount;++i)
					{
						if(rowInfo[i].isStretch)
						{
							rowInfo[i].miniSize=std::max<unsigned int>(rowInfo[i].miniSize,averageHeight);
						}
					}
				}
				else
				{
					unsigned int averageAppend(heightAvailable/rowCount);
					for(size_t i=0;i<rowCount;++i)
					{
						rowInfo[i].miniSize+=averageAppend;
					}
				}
			}

			int tempX=left+origin.x;
			int tempY=top+origin.y;

			for(size_t i=0;i<rowCount;++i)
			{
				for(size_t e=0;e<columnCount;++e)
				{
					Util::Position Cposition(tempX,tempY);
					Util::Size Carea(columnInfo[e].miniSize,rowInfo[i].miniSize);
					orderComponent(static_cast<unsigned int>(i),static_cast<unsigned int>(e),Cposition,Carea);
					tempX+=columnInfo[e].miniSize+spacer;
				}
				tempX=left+origin.x;
				tempY+=spacer+rowInfo[i].miniSize;
			}

			delete [] columnInfo;
			delete [] rowInfo;
		}

		void GirdLayout::orderComponent(unsigned int row,unsigned int column,Util::Position &origin,Util::Size &area)
		{
			struct Alignment component=alignment[row][column];
			if(component.component)
			{
				if(component.HStyle==Widgets::Element::Stretch)
				{
					component.component->size.width=area.width;
					component.component->position.x=origin.x;
				}
				else
				{
					switch(component.HAlignment)
					{
						case HLeft:
						{
							component.component->position.x=origin.x;
							break;
						}
						case HCenter:
						{
							component.component->position.x=static_cast<int>(origin.x+(area.width-component.width)*0.5f);
							break;
						}
						case HRight:
						{
							component.component->position.x=origin.x+(area.width-component.width);
							break;
						}
					}
				}

				if(component.VStyle==Widgets::Element::Stretch)
				{
					component.component->size.height=area.height;	
					component.component->position.y=origin.y;

				}
				else
				{
					switch(component.VAlignment)
					{
						case VTop:
						{
							component.component->position.y=origin.y;
							break;
						}
						case VCenter:
						{
							component.component->position.y=static_cast<int>(origin.y+(area.height-component.height)*0.5f);
							break;
						}
						case VBottom:
						{
							component.component->position.y=origin.y+(area.height-component.height);
							break;
						}
					}
				}

				component.component->pack();
			}
		}

		Util::Size GirdLayout::getPreferedSize()
		{
			return Util::Size();
		}
	}
}