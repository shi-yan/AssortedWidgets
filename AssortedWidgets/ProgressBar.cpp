#include "ProgressBar.h"

namespace AssortedWidgets
{
	namespace Widgets
    {
        ProgressBar::ProgressBar(void)
            :m_type(Horizontal),
              m_value(0.0f),
              m_min(0.0f),
              m_max(100.0f),
              m_POfSlider(0)
		{
            m_size=getPreferedSize();
            if(m_type==Horizontal)
			{
                m_horizontalStyle=Element::Stretch;
                m_verticalStyle=Element::Fit;
			}
            else if(m_type==Vertical)
			{
                m_horizontalStyle=Element::Fit;
                m_verticalStyle=Element::Stretch;
			}
			pack();
		}

        ProgressBar::ProgressBar(int _type)
            :m_type(_type),
              m_value(0.0f),
              m_min(0.0f),
              m_max(100.0f),
              m_POfSlider(0)
		{
            m_size=getPreferedSize();
            if(m_type==Horizontal)
			{
                m_horizontalStyle=Element::Stretch;
                m_verticalStyle=Element::Fit;
			}
            else if(m_type==Vertical)
			{
                m_horizontalStyle=Element::Fit;
                m_verticalStyle=Element::Stretch;
			}
			pack();
		}

        ProgressBar::ProgressBar(float _min,float _max,int _type)
            :m_type(_type),
              m_value(0.0f),
              m_min(_min),
              m_max(_max),
              m_POfSlider(0)
		{
            m_size=getPreferedSize();
            if(m_type==Horizontal)
			{
                m_horizontalStyle=Element::Stretch;
                m_verticalStyle=Element::Fit;
			}
            else if(m_type==Vertical)
			{
                m_horizontalStyle=Element::Fit;
                m_verticalStyle=Element::Stretch;
			}
			pack();
		}

        ProgressBar::ProgressBar(float _min,float _max,float _value,int _type)
            :m_type(_type),
              m_value(0),
              m_min(_min),
              m_max(_max),
              m_POfSlider(0)
		{
			setValue(_value);
            m_size=getPreferedSize();
            if(m_type==Horizontal)
			{
                m_horizontalStyle=Element::Stretch;
                m_verticalStyle=Element::Fit;
			}
            else if(m_type==Vertical)
			{
                m_horizontalStyle=Element::Fit;
                m_verticalStyle=Element::Stretch;
			}
			pack();
		}

		ProgressBar::~ProgressBar(void)
		{
		}

		void ProgressBar::pack()
		{
            if(m_type==Horizontal)
			{
                m_POfSlider=static_cast<int>(m_value*m_size.m_width);
			}
            else if(m_type==Vertical)
			{
                m_POfSlider=static_cast<int>(m_value*m_size.m_height);
			}			
		}
	}
}
