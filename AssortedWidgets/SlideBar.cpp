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
			:type(_type),minV(0.0f),maxV(100.0f),value(0.0f)
		{
			if(type==Horizontal)
			{
				slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.width=10;
                m_size.height=20;
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-4)*0.1f),4);
                slider->m_size.height=16;
                slider->m_position.x=static_cast<int>(((m_size.width-4)-slider->m_size.width)*value+2);
                slider->m_position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.width=20;
                m_size.height=10;
                slider->m_size.width=16;
                slider->m_size.height=std::max<unsigned int>(static_cast<unsigned int>((m_size.height-4)*0.1f),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-4)-slider->m_size.height)*value+2);
				slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}
		
		SlideBar::SlideBar(float _minV,float _maxV,int _type):minV(_minV),maxV(_maxV),type(_type),value(0)
		{
			if(type==Horizontal)
			{
				slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.width=10;
                m_size.height=20;
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-4)*0.1f),4);
                slider->m_size.height=16;
                slider->m_position.x=static_cast<int>(((m_size.width-4)-slider->m_size.width)*value+2);
                slider->m_position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.width=20;
                m_size.height=10;
                slider->m_size.width=16;
                slider->m_size.height=std::max<unsigned int>(static_cast<unsigned int>((m_size.height-4)*0.1f),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-4)-slider->m_size.height)*value+2);
				slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}

		SlideBar::SlideBar(float _minV,float _maxV,float _value,int _type):minV(_minV),maxV(_maxV),type(_type),value(0)
		{
			setValue(_value);
			if(type==Horizontal)
			{
				slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.width=10;
                m_size.height=20;
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-4)*0.1f),4);
                slider->m_size.height=16;
                slider->m_position.x=static_cast<int>(((m_size.width-4)-slider->m_size.width)*value+2);
                slider->m_position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.width=20;
                m_size.height=10;
                slider->m_size.width=16;
                slider->m_size.height=std::max<unsigned int>(static_cast<unsigned int>((m_size.height-4)*0.1f),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-4)-slider->m_size.height)*value+2);
				slider->setSlideBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(SlideBar::mousePressed));
		}

		void SlideBar::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(slider->isIn(mx,my))
			{
				Event::MouseEvent event(slider,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
				slider->processMousePressed(event);
				return;
			}
		}

		void SlideBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintSlideBar(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
			slider->paint();
			Util::Graphics::getSingleton().popPosition();
		}

		void SlideBar::pack()
		{
			if(type==Horizontal)
			{
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-4)*0.1f),4);
                slider->m_size.height=16;
                slider->m_position.x=static_cast<int>(((m_size.width-4)-slider->m_size.width)*value+2);
                slider->m_position.y=2;
			}
			else if(type==Vertical)
			{
                slider->m_size.width=16;
                slider->m_size.height=std::max<unsigned int>(static_cast<int>((m_size.height-4)*0.1f),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-4)-slider->m_size.height)*value+2);
			}
		}

		SlideBar::~SlideBar(void)
		{
			delete slider;
		}
	}
}
