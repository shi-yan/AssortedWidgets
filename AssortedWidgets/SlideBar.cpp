#include "SlideBar.h"
#include "ContainerElement.h"
#include "SlideBarSlider.h"
#include "ThemeEngine.h"
#include "Graphics.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		SlideBar::SlideBar(int _type)
            :m_type(_type),
              m_value(0.0f),
              m_minV(0.0f),
              m_maxV(100.0f)
		{
            if(m_type==Horizontal)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.m_width=10;
                m_size.m_height=20;
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-4)*0.1f),4);
                m_slider->m_size.m_height=16;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-4)-m_slider->m_size.m_width)*m_value+2);
                m_slider->m_position.y=2;
                m_slider->setSlideBar(this);
			}
            else if(m_type==Vertical)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.m_width=20;
                m_size.m_height=10;
                m_slider->m_size.m_width=16;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_height-4)*0.1f),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-4)-m_slider->m_size.m_height)*m_value+2);
                m_slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}
		
        SlideBar::SlideBar(float _minV,float _maxV,int _type)
            : m_type(_type),
              m_value(0),
              m_minV(_minV),
              m_maxV(_maxV)
		{
            if(m_type==Horizontal)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.m_width=10;
                m_size.m_height=20;
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-4)*0.1f),4);
                m_slider->m_size.m_height=16;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-4)-m_slider->m_size.m_width)*m_value+2);
                m_slider->m_position.y=2;
                m_slider->setSlideBar(this);
			}
            else if(m_type==Vertical)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.m_width=20;
                m_size.m_height=10;
                m_slider->m_size.m_width=16;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_height-4)*0.1f),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-4)-m_slider->m_size.m_height)*m_value+2);
                m_slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}

        SlideBar::SlideBar(float _minV,float _maxV,float _value,int _type)
            :m_type(_type),m_value(0),m_minV(_minV),m_maxV(_maxV)
		{
			setValue(_value);
            if(m_type==Horizontal)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.m_width=10;
                m_size.m_height=20;
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-4)*0.1f),4);
                m_slider->m_size.m_height=16;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-4)-m_slider->m_size.m_width)*m_value+2);
                m_slider->m_position.y=2;
                m_slider->setSlideBar(this);
			}
            else if(m_type==Vertical)
			{
                m_slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.m_width=20;
                m_size.m_height=10;
                m_slider->m_size.m_width=16;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_height-4)*0.1f),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-4)-m_slider->m_size.m_height)*m_value+2);
                m_slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}

		void SlideBar::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_slider->isIn(mx,my))
			{
                Event::MouseEvent event(m_slider,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_slider->processMousePressed(event);
				return;
			}
		}

		void SlideBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintSlideBar(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
            m_slider->paint();
			Util::Graphics::getSingleton().popPosition();
		}

		void SlideBar::pack()
		{
            if(m_type==Horizontal)
			{
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-4)*0.1f),4);
                m_slider->m_size.m_height=16;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-4)-m_slider->m_size.m_width)*m_value+2);
                m_slider->m_position.y=2;
			}
            else if(m_type==Vertical)
			{
                m_slider->m_size.m_width=16;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<int>((m_size.m_height-4)*0.1f),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-4)-m_slider->m_size.m_height)*m_value+2);
			}
		}

		SlideBar::~SlideBar(void)
		{
            delete m_slider;
		}
	}
}
