#pragma once
#include "DragAble.h"
#include "SlideBar.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class SlideBarSlider:public DragAble
		{
		private:
			SlideBar *parent;
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};
		private:
			int type;
		public:
			int getType()
			{
				return type;
			}
			void setSlideBar(SlideBar *_parent)
			{
				parent=_parent;
            }
			SlideBarSlider(int _type);
			Util::Size getPreferedSize()
			{
                return m_size;
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintSlideBarSlider(this);
            }
						
			void dragReleased(const Event::MouseEvent &e)
            {}

			void dragMoved(int offsetX,int offsetY)
			{
				if(type==Horizontal)
				{
                    m_position.x+=offsetX;
                    if(m_position.x<2)
					{
                        m_position.x=2;
					}
                    else if(m_position.x>static_cast<int>(parent->m_size.width-2-m_size.width))
					{
                        m_position.x=parent->m_size.width-2-m_size.width;
					}
                    parent->setPercent(std::min<float>(1.0f,static_cast<float>(m_position.x-2)/static_cast<float>(parent->m_size.width-4-m_size.width)));
				}
				else if(type==Vertical)
				{
                    m_position.y+=offsetY;
                    if(m_position.y<2)
					{
                        m_position.y=2;
					}
                    else if(m_position.y>static_cast<int>(parent->m_size.height-2-m_size.height))
					{
                        m_position.y=parent->m_size.height-2-m_size.height;
					}
                    parent->setPercent(std::min<float>(1.0f,static_cast<float>(m_position.y-2)/static_cast<float>(parent->m_size.height-4-m_size.height)));
				}
//				parent->onValueChanged();
			};

		public:
			~SlideBarSlider(void);
		};
	}
}
