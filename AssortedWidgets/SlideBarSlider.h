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
            SlideBar *m_parent;
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};
		private:
            int m_type;
		public:
            int getType() const
			{
                return m_type;
			}
			void setSlideBar(SlideBar *_parent)
			{
                m_parent=_parent;
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
						
            void dragReleased(const Event::MouseEvent &)
            {}

			void dragMoved(int offsetX,int offsetY)
			{
                if(m_type==Horizontal)
				{
                    m_position.x+=offsetX;
                    if(m_position.x<2)
					{
                        m_position.x=2;
					}
                    else if(m_position.x>static_cast<int>(m_parent->m_size.m_width-2-m_size.m_width))
					{
                        m_position.x=m_parent->m_size.m_width-2-m_size.m_width;
					}
                    m_parent->setPercent(std::min<float>(1.0f,static_cast<float>(m_position.x-2)/static_cast<float>(m_parent->m_size.m_width-4-m_size.m_width)));
				}
                else if(m_type==Vertical)
				{
                    m_position.y+=offsetY;
                    if(m_position.y<2)
					{
                        m_position.y=2;
					}
                    else if(m_position.y>static_cast<int>(m_parent->m_size.m_height-2-m_size.m_height))
					{
                        m_position.y=m_parent->m_size.m_height-2-m_size.m_height;
					}
                    m_parent->setPercent(std::min<float>(1.0f,static_cast<float>(m_position.y-2)/static_cast<float>(m_parent->m_size.m_height-4-m_size.m_height)));
				}
//				parent->onValueChanged();
            }

		public:
			~SlideBarSlider(void);
		};
	}
}
