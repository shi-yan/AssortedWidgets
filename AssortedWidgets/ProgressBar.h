#pragma once
#include "ContainerElement.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ProgressBar:public Element
		{
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};

		private:
            int m_type;
            float m_value;
            float m_min;
            float m_max;
            unsigned int m_POfSlider;
		public:
            int getType() const
			{
                return m_type;
            }

            float getValue() const
			{
                return m_min+(m_max-m_min)*m_value;
            }

            unsigned int getPOfSlider() const
			{
                return m_POfSlider;
            }

			void setValue(float _value)
			{
                if(_value>=m_min && _value<=m_max)
				{
                    m_value=(_value-m_min)/(m_max-m_min);
                    if(m_type==Horizontal)
					{
                        m_POfSlider=static_cast<unsigned int>(m_value*m_size.m_width);
					}
                    else if(m_type==Vertical)
					{
                        m_POfSlider=static_cast<unsigned int>(m_value*m_size.m_height);
					}
				}
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
			
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintProgressBar(this);
            }

			void pack();
			ProgressBar(void);
			ProgressBar(int _type);
			ProgressBar(float _min,float _max,int _type=Horizontal);
			ProgressBar(float _min,float _max,float _value,int _type=Horizontal);
		public:
			~ProgressBar(void);
		};
	}
}
