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
			ScrollBar *parent;
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
			void setScrollBar(ScrollBar *_parent)
			{
				parent=_parent;
			};
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
            {}

			void dragMoved(int offsetX,int offsetY)
			{
				if(type==Horizontal)
				{
                    m_position.x+=offsetX;
                    if(m_position.x<17)
					{
                        m_position.x=17;
					}
                    else if(m_position.x>static_cast<int>(parent->m_size.width-17-m_size.width))
					{
                        m_position.x=parent->m_size.width-17-m_size.width;
					}
                    parent->setValue(static_cast<float>(m_position.x-17)/static_cast<float>(parent->m_size.width-34-m_size.width));
				}
				else if(type==Vertical)
				{
                    m_position.y+=offsetY;
                    if(m_position.y<17)
					{
                        m_position.y=17;
					}
                    else if(m_position.y>static_cast<int>(parent->m_size.height-17-m_size.height))
					{
                        m_position.y=parent->m_size.height-17-m_size.height;
					}
                    parent->setValue(static_cast<float>(m_position.y-17)/static_cast<float>(parent->m_size.height-34-m_size.height));
				}
				parent->onValueChanged();
			};

		public:
			~ScrollBarSlider(void);
		};
	}
}
