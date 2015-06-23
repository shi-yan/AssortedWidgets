#pragma once
#include "ContainerElement.h"
#include "ScrollPanel.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ScrollBarButton;
		class ScrollBarSlider;

		class ScrollBar:public Element
		{
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};
		private:
			ScrollBarButton *min;
			ScrollBarButton *max;
			ScrollBarSlider *slider;
			ScrollPanel *parent;
			int type;
			float value;
		public:
			void setScrollPanel(ScrollPanel *_parent)
			{
				parent=_parent;
			};
			void onValueChanged();
			ScrollBar(int _type);
			float getValue()
			{
				return value;
			}
			void setValue(float _value)
			{
				value=_value;
			//	printf("%f",value);
			}
			int getType()
			{
				return type;
			};
			Util::Size getPreferedSize()
			{
				if(type==Horizontal)
				{
					return Util::Size(40,15);
				}
				else if(type==Vertical)
				{
					return Util::Size(15,40);
				}
				return Util::Size();
			};
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);

			void onMinReleased(const Event::MouseEvent &e);
			void onMaxReleased(const Event::MouseEvent &e);
			void pack();
		public:
			~ScrollBar(void);
		};
	}
}