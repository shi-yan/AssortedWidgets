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
            ScrollBarButton *m_min;
            ScrollBarButton *m_max;
            ScrollBarSlider *m_slider;
            ScrollPanel *m_parent;
            int m_type;
            float m_value;
		public:
			void setScrollPanel(ScrollPanel *_parent)
			{
                m_parent=_parent;
            }
			void onValueChanged();
			ScrollBar(int _type);
            float getValue() const
			{
                return m_value;
			}
			void setValue(float _value)
			{
                m_value=_value;
			//	printf("%f",value);
			}
            int getType() const
			{
                return m_type;
            }
			Util::Size getPreferedSize()
			{
                if(m_type==Horizontal)
				{
					return Util::Size(40,15);
				}
                else if(m_type==Vertical)
				{
					return Util::Size(15,40);
				}
				return Util::Size();
            }
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
