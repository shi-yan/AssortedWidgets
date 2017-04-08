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

            unsigned int width = area.m_width - m_left - m_right;
            unsigned int height = area.m_height - m_top - m_bottom;

            int tempX = origin.x + m_left;
            int tempY = origin.y + m_top;

            (void) width;
            (void) height;
            (void) tempX;
            (void) tempY;

            unsigned int westHeight(getPreferedHeight(west,m_westFormat));
            unsigned int centerHeight(getPreferedHeight(center,m_centerFormat));
            unsigned int eastHeight(getPreferedHeight(east,m_eastFormat));
            unsigned int northHeight(getPreferedHeight(north,m_northFormat));
            unsigned int southHeight(getPreferedHeight(south,m_southFormat));

            unsigned int heightAvailable(area.m_height-m_top-m_bottom-m_spacer-m_spacer);
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
            unsigned int northWidth(getPreferedWidth(north,m_northFormat));
            unsigned int southWidth(getPreferedWidth(south,m_southFormat));
            unsigned int eastWidth(getPreferedWidth(east,m_eastFormat));
            unsigned int westWidth(getPreferedWidth(west,m_westFormat));
            unsigned int centerWidth(getPreferedWidth(center,m_centerFormat));

            unsigned int widthAvailable(area.m_width-m_left-m_right);
            widthAvailable=std::max<unsigned int>(widthAvailable,std::max<unsigned int>(westWidth+eastWidth+centerWidth+m_spacer+m_spacer,std::max<unsigned int>(northWidth,southWidth)));
			northWidth=southWidth=widthAvailable;
            widthAvailable-=m_spacer+m_spacer;

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

            Util::Position northPosition(origin.x+m_left,origin.y+m_top);
			Util::Size northArea(northWidth,northHeight);

            orderComponents(north,m_northHAlignment,m_northVAlignment,m_northFormat,northPosition,northArea);

            Util::Position southPosition(origin.x+m_left,origin.y+m_top+m_spacer+centerHeight+m_spacer+northHeight);
			Util::Size southArea(southWidth,southHeight);
            orderComponents(south,m_southHAlignment,m_southVAlignment,m_southFormat,southPosition,southArea);

            Util::Position westPosition(origin.x+m_left,origin.y+m_top+northHeight+m_spacer);
			Util::Size westArea(westWidth,westHeight);
            orderComponents(west,m_westHAlignment,m_westVAlignment,m_westFormat,westPosition,westArea);



            Util::Position eastPosition(origin.x+m_left+westWidth+m_spacer+centerWidth+m_spacer,origin.y+m_top+northHeight+m_spacer);
			Util::Size eastArea(eastWidth,eastHeight);

            m_testNorthX=static_cast<float>(eastPosition.x);
            m_testNorthY=static_cast<float>(eastPosition.y);
            m_testNorthWidth=static_cast<float>(eastArea.m_width);
            m_testNorthHeight=static_cast<float>(eastArea.m_height);

            orderComponents(east,m_eastHAlignment,m_eastVAlignment,m_eastFormat,eastPosition,eastArea);

            Util::Position centerPosition(origin.x+m_left+m_spacer+westWidth,origin.y+m_spacer+northHeight+m_top);
			Util::Size centerArea(centerWidth,centerHeight);

            orderComponents(center,m_centerHAlignment,m_centerVAlignment,m_centerFormat,centerPosition,centerArea);
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
                                    widthTakenUp+=perfectSize.m_width;
								}
							}

                            unsigned int widthAvailable(area.m_width-m_spacer*(list.size()-1)-widthTakenUp);
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
                                    (*iter)->m_position.x=tempX;
                                    (*iter)->m_size.m_width=perfectSize.m_width;
                                    tempX+=m_spacer+perfectSize.m_width;
								}
								else if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
								{
                                    (*iter)->m_position.x=tempX;
                                    (*iter)->m_size.m_width=averageWidth;
                                    tempX+=m_spacer+averageWidth;
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
                                    widthTakenUp+=perfectSize.m_width;
								}
							}

                            unsigned int widthAvailable(area.m_width-m_spacer*(list.size()-1)-widthTakenUp);
							unsigned int averageWidth(0);
							if(strechSegment)
							{
								averageWidth=widthAvailable/strechSegment;
							}

                            int tempX=origin.x+area.m_width;

							for(int i=static_cast<int>(list.size()-1);i>=0;--i)
							{
								Widgets::Element *iter=list[i];
								Util::Size perfectSize=iter->getPreferedSize();
								if(iter->getHorizontalStyle()==Widgets::Element::Fit)
								{
                                    tempX-=perfectSize.m_width;
                                    iter->m_position.x=tempX;
                                    iter->m_size.m_width=perfectSize.m_width;
                                    tempX-=m_spacer;
								}
								else if(iter->getHorizontalStyle()==Widgets::Element::Stretch)
								{
									tempX-=averageWidth;
                                    iter->m_position.x=tempX;
                                    iter->m_size.m_width=averageWidth;
                                    tempX-=m_spacer;
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
                                    widthTakenUp+=perfectSize.m_width;
								}
							}

							if(isStretch)
							{
                                unsigned int widthAvailable(area.m_width-m_spacer*(list.size()-1)-widthTakenUp);
								unsigned int averageWidth=widthAvailable/strechSegment;
								int tempX=origin.x;
				
								for(iter=list.begin();iter<list.end();++iter)
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
									if((*iter)->getHorizontalStyle()==Widgets::Element::Fit)
									{
                                        (*iter)->m_position.x=tempX;
                                        (*iter)->m_size.m_width=perfectSize.m_width;
                                        tempX+=m_spacer+perfectSize.m_width;
									}
									else if((*iter)->getHorizontalStyle()==Widgets::Element::Stretch)
									{
                                        (*iter)->m_position.x=tempX;
                                        (*iter)->m_size.m_width=averageWidth;
                                        tempX+=m_spacer+averageWidth;
									}
								}
							}
							else
							{
                                widthTakenUp+=m_spacer*(list.size()-1);
                                int tempX=static_cast<int>(origin.x+area.m_width*0.5f-widthTakenUp*0.5f);
								for(iter=list.begin();iter<list.end();++iter)
								{
									Util::Size perfectSize=(*iter)->getPreferedSize();
                                    (*iter)->m_position.x=tempX;
                                    (*iter)->m_size.m_width=perfectSize.m_width;
                                    tempX+=m_spacer+perfectSize.m_width;
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
                                    (*iter)->m_position.y=tempY;
                                    (*iter)->m_size.m_height=area.m_height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
                                    (*iter)->m_position.y=tempY;
                                    (*iter)->m_size.m_height=perfectSize.m_height;
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
                                    (*iter)->m_position.y=tempY;
                                    (*iter)->m_size.m_height=area.m_height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
                                    (*iter)->m_position.y=tempY+area.m_height-perfectSize.m_height;
                                    (*iter)->m_size.m_height=perfectSize.m_height;
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
                                    (*iter)->m_position.y=tempY;
                                    (*iter)->m_size.m_height=area.m_height;
								}
								else if((*iter)->getVerticalStyle()==Widgets::Element::Fit)
								{
                                    (*iter)->m_position.y=static_cast<int>(tempY+area.m_height*0.5-perfectSize.m_height*0.5);
                                    (*iter)->m_size.m_height=perfectSize.m_height;
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
                        resultWidth+=m_spacer+perfectSize.m_width;
					}
                    resultWidth-=m_spacer;
				}
				else if(format==vertical)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
                        resultWidth=std::max<unsigned int>(resultWidth,perfectSize.m_width);
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
                        resultHeight=std::max<unsigned int>(resultHeight,perfectSize.m_height);
					}
				}
				else if(format==vertical)
				{
					std::vector<Widgets::Element*>::iterator iter;
					for(iter=list.begin();iter<list.end();++iter)
					{
						Util::Size perfectSize=(*iter)->getPreferedSize();
                        resultHeight+=m_spacer+perfectSize.m_height;
					}
                    resultHeight-=m_spacer;
				}
			}
			return resultHeight;
		}

        Util::Size BorderLayout::getPreferedSize() const
		{
			return Util::Size();
		}
	}
}
