#pragma once
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class SlideBarSlider;

		class SlideBar:public Element
		{
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};

		private:
            SlideBarSlider *m_slider;
            int m_type;
            float m_value;
            float m_minV;
            float m_maxV;

		public:
            float getValue() const
			{
                return (m_maxV-m_minV)*m_value+m_minV;
            }
            float getMax() const
			{
                return m_maxV;
            }
			//void onValueChanged();
			void setValue(float _value)
			{
                if(_value>=m_minV && _value<=m_maxV)
				{
                    m_value=(_value-m_minV)/(m_maxV-m_minV);
				}
            }
			void setPercent(float _value)
			{
                m_value=_value;
            }
			SlideBar(int _type=Horizontal);
			SlideBar(float _minV,float _maxV,int _type=Horizontal);
			SlideBar(float _minV,float _maxV,float _value,int _type=Horizontal);
            int getType() const
			{
                return m_type;
            }
			Util::Size getPreferedSize()
			{
                if(m_type==Horizontal)
				{
					return Util::Size(10,20);
				}
                else if(m_type==Vertical)
				{
					return Util::Size(20,10);
				}
				return Util::Size();
            }
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void pack();
		public:
			~SlideBar(void);
		};
	}
}
