#pragma once
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ScrollBar;

		class ScrollPanel:public Element
		{
		public:
			enum ScrollStyle
			{
				Auto,
				Never
			};
		private:
            Element *m_content;
            unsigned int m_offsetX;
            unsigned int m_offsetY;
            unsigned int m_offsetXMax;
            unsigned int m_offsetYMax;
            unsigned int m_scissorWidth;
            unsigned int m_scissorHeight;
            int m_horizontalScrollStyle;
            int m_verticalScrollStyle;
            ScrollBar *m_horizontalBar;
            ScrollBar *m_verticalBar;
            bool m_horizontalBarShow;
            bool m_verticalBarShow;
			
		public:
			void onValueChanged(ScrollBar *scrollBar);
            bool isHorizontalBarShow() const
			{
                return m_horizontalBarShow;
            }
            bool isVerticalBarShow() const
			{
                return m_verticalBarShow;
            }
			void setHorizontalScrollStyle(int _horizontalScrollStyle)
			{
                m_horizontalScrollStyle=_horizontalScrollStyle;
			}
			void setVerticalScrollStyle(int _verticalScrollStyle)
			{
                m_verticalScrollStyle=_verticalScrollStyle;
			}
            int getHorizontalScrollStyle() const
			{
                return m_horizontalScrollStyle;
			}
            int getVerticalScrollStyle() const
			{
                return m_verticalScrollStyle;
			}
            unsigned int getOffsetX() const
			{
                return m_offsetX;
            }
            unsigned int getOffsetY() const
			{
                return m_offsetY;
            }
			void setContent(Element *_content)
			{
                m_content=_content;
                m_offsetXMax=m_content->m_size.m_width-(m_size.m_width-17);
                m_offsetYMax=m_content->m_size.m_height-(m_size.m_height-17);
			}
			void removeContent()
			{
                m_content=0;
			}
			ScrollPanel(void);
			Util::Size getPreferedSize()
			{
				return Util::Size(60,60);
            }
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);

			void mouseMoved(const Event::MouseEvent &e);

			void pack();
		public:
			~ScrollPanel(void);
		};
	}
}
