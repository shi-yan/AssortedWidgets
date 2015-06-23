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
			Element *content;
			unsigned int offsetX;
			unsigned int offsetY;
			unsigned int offsetXMax;
			unsigned int offsetYMax;
			unsigned int scissorWidth;
			unsigned int scissorHeight;
			int horizontalScrollStyle;
			int verticalScrollStyle;
			ScrollBar *horizontalBar;
			ScrollBar *verticalBar;
			bool horizontalBarShow;
			bool verticalBarShow;
			
		public:
			void onValueChanged(ScrollBar *scrollBar);
			bool isHorizontalBarShow()
			{
				return horizontalBarShow;
			};
			bool isVerticalBarShow()
			{
				return verticalBarShow;
			};
			void setHorizontalScrollStyle(int _horizontalScrollStyle)
			{
				horizontalScrollStyle=_horizontalScrollStyle;
			}
			void setVerticalScrollStyle(int _verticalScrollStyle)
			{
				verticalScrollStyle=_verticalScrollStyle;
			}
			int getHorizontalScrollStyle()
			{
				return horizontalScrollStyle;
			}
			int getVerticalScrollStyle()
			{
				return verticalScrollStyle;
			}
			unsigned int getOffsetX()
			{
				return offsetX;
			};
			unsigned int getOffsetY()
			{
				return offsetY;
			};
			void setContent(Element *_content)
			{
				content=_content;
				offsetXMax=content->size.width-(size.width-17);
				offsetYMax=content->size.height-(size.height-17);
			}
			void removeContent()
			{
				content=0;
			}
			ScrollPanel(void);
			Util::Size getPreferedSize()
			{
				return Util::Size(60,60);
			};
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