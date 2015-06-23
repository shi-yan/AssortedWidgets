#include "BorderLayout.h"
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Layout
	{
		BorderLayout::~BorderLayout(void)
		{
		}

		void BorderLayout::updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area)
		{
			std::vector<Widgets::Element*> north;
			int northHStyle(Widgets::Element::Any);
			int northVStyle(Widgets::Element::Any);
			std::vector<Widgets::Element*> south;
			int southHStyle(Widgets::Element::Any);
			int southVStyle(Widgets::Element::Any);
			std::vector<Widgets::Element*> west;
			int westHStyle(Widgets::Element::Any);
			int westVStyle(Widgets::Element::Any);
			std::vector<Widgets::Element*> east;
			int eastHStyle(Widgets::Element::Any);
			int eastVStyle(Widgets::Element::Any);
			std::vector<Widgets::Element*> center;
			int centerHStyle(Widgets::Element::Any);
			int centerVStyle(Widgets::Element::Any);

			north.reserve(20);
			south.reserve(20);
			west.reserve(20);
			east.reserve(20);
			center.reserve(20);

			std::vector<Widgets::Element*>::iterator iter;
			for(iter=componentList.begin();iter<componentList.end();++iter)
			{
				switch((*iter)->getLayoutProperty())
				{
					case North:
						{
							north.push_back((*iter));
							northHStyle=std::max<int>(northHStyle,(*iter)->getHorizontalStyle());
							northVStyle=std::max<int>(northVStyle,(*iter)->getVerticalStyle());
							break;
						}
					case South:
						{
							south.push_back((*iter));
							southHStyle=std::max<int>(southHStyle,(*iter)->getHorizontalStyle());
							southVStyle=std::max<int>(southVStyle,(*iter)->getVerticalStyle());
							break;
						}
					case West:
						{
							west.push_back((*iter));
							westHStyle=std::max<int>(westHStyle,(*iter)->getHorizontalStyle());
							westVStyle=std::max<int>(westVStyle,(*iter)->getVerticalStyle());
							break;
						}
					case East:
						{
							east.push_back((*iter));
							eastHStyle=std::max<int>(eastHStyle,(*iter)->getHorizontalStyle());
							eastVStyle=std::max<int>(eastVStyle,(*iter)->getVerticalStyle());
							break;
						}
					case Center:
						{
							center.push_back((*iter));
							centerHStyle=std::max<int>(centerHStyle,(*iter)->getHorizontalStyle());
							centerVStyle=std::max<int>(centerVStyle,(*iter)->getVerticalStyle());
							break;
						}
				}
			}

			unsigned int width=area.width-left-right;
			unsigned int height=area.height-top-bottom;

			int tempX=origin.x+left;
			int tempY=origin.y+top;

			//计算所有5个区域的高度
			unsigned int westHeight(getPreferedHeight(west,westFormat));
			unsigned int centerHeight(getPreferedHeight(center,centerFormat));
			unsigned int eastHeight(getPreferedHeight(east,eastFormat));
			unsigned int northHeight(getPreferedHeight(north,northFormat));
			unsigned int southHeight(getPreferedHeight(south,southFormat));

			unsigned int heightAvailable(area.height-top-bottom-spacer-spacer);
			heightAvailable=std::max<unsigned int>(heightAvailable,std::max<unsigned int>(std::max<unsigned int>(westHeight,eastHeight),centerHeight)+northHeight+southHeight);
			int strechAreaCount(1);
			if(northVStyle==Widgets::Element::Stretch)
			{
				++strechAreaCount;
			}
			else
			{
				heightAvailable-=northHeight;
			}

			if(southVStyle==Widgets::Element::Stretch)
			{
				++strechAreaCount;
			}
			else
			{
				heightAvailable-=southHeight;
			}

			unsigned int averageHeight=heightAvailable/strechAreaCount;
			if(northVStyle==Widgets::Element::Stretch)
			{
				northHeight=std::max<unsigned int>(northHeight,averageHeight);
			}
			if(southVStyle==Widgets::Element::Stretch)
			{
				southHeight=std::max<unsigned int>(southHeight,averageHeight);
			}

			westHeight=centerHeight=eastHeight=std::max<unsigned int>(std::max<unsigned int>(westHeight,eastHeight),std::max<unsigned int>(centerHeight,averageHeight));

			//计算所有5个区域的宽度
			unsigned int northWidth(getPreferedWidth(north,northFormat));
			unsigned int southWidth(getPreferedWidth(south,southFormat));
			unsigned int eastWidth(getPreferedWidth(east,eastFormat));
			unsigned int westWidth(getPreferedWidth(west,westFormat));
			unsigned int centerWidth(getPreferedWidth(center,centerFormat));

			unsigned int widthAvailable(area.width-left-right);
			widthAvailable=std::max<unsigned int>(widthAvailable,std::max<unsigned int>(westWidth+eastWidth+centerWidth+spacer+spacer,std::max<unsigned int>(northWidth,southWidth)));
			northWidth=southWidth=widthAvailable;
			widthAvailable-=spacer+spacer;

			strechAreaCount=1;
			if(westHStyle==Widgets::Element::Stretch)
			{
				++strechAreaCount;
			}
			else
			{
				widthAvailable-=westWidth;
			}

			if(eastHStyle==Widgets::Element::Stretch)
			{
				++strechAreaCount;
			}
			else
			{
				widthAvailable-=eastWidth;
			}

			unsigned int averageWidth=widthAvailable/strechAreaCount;
			if(westHStyle==Widgets::Element::Stretch)
			{
				westWidth=averageWidth;
			}
			if(eastHStyle==Widgets::Element::Stretch)
			{
				eastWidth=averageWidth;
			}
			centerWidth=std::max<unsigned int>(averageWidth,centerWidth);

			Util::Position northPosition(origin.x+left,origin.y+top);
			Util::Size northArea(northWidth,northHeight);

			orderComponents(north,northHAlignment,northVAlignment,northFormat,northPosition,northArea);

			Util::Position southPosition(origin.x+left,origin.y+top+spacer+centerHeight+spacer+northHeight);
			Util::Size southArea(southWidth,southHeight);
			orderComponents(south,southHAlignment,southVAlignment,southFormat,southPosition,southArea);

			Util::Position westPosition(origin.x+left,origin.y+top+northHeight+spacer);
			Util::Size westArea(westWidth,westHeight);
			orderComponents(west,westHAlignment,westVAlignment,westFormat,westPosition,westArea);



			Util::Position eastPosition(origin.x+left+westWidth+spacer+centerWidth+spacer,origin.y+top+northHeight+spacer);
			Util::Size eastArea(eastWidth,eastHeight);

			testNorthX=static_cast<float>(eastPosition.x);
			testNorthY=static_cast<float>(eastPosition.y);
			testNorthWidth=static_cast<float>(eastArea.width);
			testNorthHeight=static_cast<float>(eastArea.height);

			orderComponents(east,eastHAlignment,eastVAlignment,eastFormat,eastPosition,eastArea);

			Util::Position centerPosition(origin.x+left+spacer+westWidth,origin.y+spacer+northHeight+top);
			Util::Size centerArea(centerWidth,centerHeight);

			orderComponents(center,centerHAlignment,centerVAlignment,centerFormat,centerPosition,centerArea);
		}

		void BorderLayout::orderComponents(std::vector<Widgets::Element*> &list,int HAlignment,int VAlignment,int format,Util::Position &origin,Util::Size &area)
		{
			if(!list.empty())
			{
				if(format==horizontal)
				{
					switch(HAlignment)
					{
						case HLeft:
						{
							int strechSegment(0);
							unsigned int widthTakenUp(0);
							std::vector<Widgets::Element*>::iterator iter;
							for(iter=list.begin();iter<list.end();++iter)
							{
								if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									++strechSegment;
								}
								else
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									widthTakenUp+=perfectSize.width;
								}
							}

							unsigned int widthAvailable(area.width-spacer*(list.size()-1)-widthTakenUp);
							unsigned int averageWidth(0);
							if(strechSegment)
							{
								averageWidth=widthAvailable/strechSegment;
							}
	
							int tempX=origin.x;
							for(iter=list.begin();iter<list.end();++iter)
							{
								Util::Size perfectSize=(*iter)->getPreferedSize();
								if((*iter)->getHorizontalStyle()==Widgets::Element::Fit)
								{
									(*iter)->position.x=tempX;
									(*iter)->size.width=perfectSize.width;
									tempX+=spacer+perfectSize.width;
								}
								else if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									(*iter)->position.x=tempX;
									(*iter)->size.width=averageWidth;
									tempX+=spacer+averageWidth;
								}
							}
							break;
						}
						case HRight:
						{
							int strechSegment(0);
							unsigned int widthTakenUp(0);
							std::vector<Widgets::Element*>::iterator iter;
							for(iter=list.begin();iter<list.end();++iter)
							{
								if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									++strechSegment;
								}
								else
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									widthTakenUp+=perfectSize.width;
								}
							}

							unsigned int widthAvailable(area.width-spacer*(list.size()-1)-widthTakenUp);
							unsigned int averageWidth(0);
							if(strechSegment)
							{
								averageWidth=widthAvailable/strechSegment;
							}

							int tempX=origin.x+area.width;

							for(int i=static_cast<int>(list.size()-1);i>=0;--i)
							{
								Widgets::Element *iter=list[i];
								Util::Size perfectSize=iter->getPreferedSize();
								if(iter->getHorizontalStyle()==Widgets::Element::Fit)
								{
									tempX-=perfectSize.width;
									iter->position.x=tempX;
									iter->size.width=perfectSize.width;
									tempX-=spacer;
								}
								else if(iter->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									tempX-=averageWidth;
									iter->position.x=tempX;
									iter->size.width=averageWidth;
									tempX-=spacer;
								}
							}
							break;
						}
						case HCenter:
						{
							bool isStretch(false);
							int strechSegment(0);
							unsigned int widthTakenUp(0);
							std::vector<Widgets::Element*>::iterator iter;
							for(iter=list.begin();iter<list.end();++iter)
							{
								if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									++strechSegment;
									isStretch=true;
								}
								else
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									widthTakenUp+=perfectSize.width;
								}
							}

							if(isStretch)
							{
								unsigned int widthAvailable(area.width-spacer*(list.size()-1)-widthTakenUp);
								unsigned int averageWidth=widthAvailable/strechSegment;
								int tempX=origin.x;
				
								for(iter=list.begin();iter<list.end();++iter)
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									if((*iter)->getHorizontalStyle()==Widgets::Element::Fit)
									{
										(*iter)->position.x=tempX;
										(*iter)->size.width=perfectSize.width;
										tempX+=spacer+perfectSize.width;
									}
									else if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
									{
										(*iter)->position.x=tempX;
										(*iter)->size.width=averageWidth;
										tempX+=spacer+averageWidth;
									}
								}
							}
							else
							{
								widthTakenUp+=spacer*(list.size()-1);
								int tempX=static_cast<int>(origin.x+area.width*0.5f-widthTakenUp*0.5f);
								for(iter=list.begin();iter<list.end();++iter)
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									(*iter)->position.x=tempX;
									(*iter)->size.width=perfectSize.width;
									tempX+=spacer+perfectSize.width;
								}
							}
							break;
						}
					}

					switch(VAlignment)
					{
						case VTop:
						{
							std::vector<Widgets::Element*>::iterator iter;
							int tempY=origin.y;
							for(iter=list.begin();iter<list.end();++iter)
							{
								Util::Size perfectSize=(*iter)->getPreferedSize();
								if((*iter)->getVerticalStyle()==Widgets::Element::Stretch)
								{
									(*iter)->position.y=tempY;
									(*iter)->size.height=area.height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
									(*iter)->position.y=tempY;
									(*iter)->size.height=perfectSize.height;
								}
							}
							break;
						}
						case VBottom:
						{
							std::vector<Widgets::Element*>::iterator iter;
							int tempY=origin.y;
							for(iter=list.begin();iter<list.end();++iter)
							{
								Util::Size perfectSize=(*iter)->getPreferedSize();
								if((*iter)->getVerticalStyle()==Widgets::Element::Stretch)
								{
									(*iter)->position.y=tempY;
									(*iter)->size.height=area.height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
									(*iter)->position.y=tempY+area.height-perfectSize.height;
									(*iter)->size.height=perfectSize.height;
								}
							}
							break;
						}
						case VCenter:
						{
							std::vector<Widgets::Element*>::iterator iter;
							int tempY=origin.y;
							for(iter=list.begin();iter<list.end();++iter)
							{
								Util::Size perfectSize=(*iter)->getPreferedSize();
								if((*iter)->getVerticalStyle()==Widgets::Element::Stretch)
								{
									(*iter)->position.y=tempY;
									(*iter)->size.height=area.height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
									(*iter)->position.y=static_cast<int>(tempY+area.height*0.5-perfectSize.height*0.5);
									(*iter)->size.height=perfectSize.height;
								}
							}
							break;
						}
					}
				}
				else if(format==vertical)
				{
					
				}

				
				std::vector<Widgets::Element*>::iterator iter;
				for(iter=list.begin();iter<list.end();++iter)
				{
					(*iter)->pack();
				}
			}
		}

		unsigned int BorderLayout::getPreferedWidth(std::vector<Widgets::Element*> &list,int format)
		{
			unsigned int resultWidth(0);
			if(!list.empty())
			{
				if(format==horizontal)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
						resultWidth+=spacer+perfectSize.width;
					}
					resultWidth-=spacer;
				}
				else if(format==vertical)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
						resultWidth=std::max<unsigned int>(resultWidth,perfectSize.width);
					}
				}
			}

			return resultWidth;
		}

		unsigned int BorderLayout::getPreferedHeight(std::vector<Widgets::Element*> &list,int format)
		{
			unsigned int resultHeight(0);
			if(!list.empty())
			{
				if(format==horizontal)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
						resultHeight=std::max<unsigned int>(resultHeight,perfectSize.height);
					}
				}
				else if(format==vertical)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
						resultHeight+=spacer+perfectSize.height;
					}
					resultHeight-=spacer;
				}
			}
			return resultHeight;
		}

		Util::Size BorderLayout::getPreferedSize()
		{
			return Util::Size();
		}
	}
}