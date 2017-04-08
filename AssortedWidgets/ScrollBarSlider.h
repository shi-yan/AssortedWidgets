#pragma once
#include "DragAble.h"
#include "ThemeEngine.h"
#include "ScrollBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ScrollBarSlider:public DragAble
		{
		private:
            ScrollBar *m_parent;
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
			void setScrollBar(ScrollBar *_parent)
			{
                m_parent=_parent;
            }
			ScrollBarSlider(int _type);
			Util::Size getPreferedSize()
			{
                return m_size;
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintScrollBarSlider(this);
            }

			void dragReleased(const Event::MouseEvent &e)
            {
                (void) e;
            }

			void dragMoved(int offsetX,int offsetY)
			{
                if(m_type==Horizontal)
				{
                    m_position.x+=offsetX;
                    if(m_position.x<17)
					{
                        m_position.x=17;
					}
                    else if(m_position.x>static_cast<int>(m_parent->m_size.m_width-17-m_size.m_width))
					{
                        m_position.x=m_parent->m_size.m_width-17-m_size.m_width;
					}
                    m_parent->setValue(static_cast<float>(m_position.x-17)/static_cast<float>(m_parent->m_size.m_width-34-m_size.m_width));
				}
                else if(m_type==Vertical)
				{
                    m_position.y+=offsetY;
                    if(m_position.y<17)
					{
                        m_position.y=17;
					}
                    else if(m_position.y>static_cast<int>(m_parent->m_size.m_height-17-m_size.m_height))
					{
                        m_position.y=m_parent->m_size.m_height-17-m_size.m_height;
					}
                    m_parent->setValue(static_cast<float>(m_position.y-17)/static_cast<float>(m_parent->m_size.m_height-34-m_size.m_height));
				}
                m_parent->onValueChanged();
            }

		public:
			~ScrollBarSlider(void);
		};
	}
}
