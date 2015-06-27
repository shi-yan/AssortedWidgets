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
				size.width=10;
				size.height=20;
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-4)*0.1f),4);
				slider->size.height=16;
				slider->position.x=static_cast<int>(((size.width-4)-slider->size.width)*value+2);
				slider->position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
				size.width=20;
				size.height=10;
				slider->size.width=16;
				slider->size.height=std::max<unsigned int>(static_cast<unsigned int>((size.height-4)*0.1f),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-4)-slider->size.height)*value+2);
				slider->setSlideBar(this);
			}

			MouseDelegate mPressed;
			mPressed.bind(this,&SlideBar::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}
		
		SlideBar::SlideBar(float _minV,float _maxV,int _type):minV(_minV),maxV(_maxV),type(_type),value(0)
		{
			if(type==Horizontal)
			{
				slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
				size.width=10;
				size.height=20;
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-4)*0.1f),4);
				slider->size.height=16;
				slider->position.x=static_cast<int>(((size.width-4)-slider->size.width)*value+2);
				slider->position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
				size.width=20;
				size.height=10;
				slider->size.width=16;
				slider->size.height=std::max<unsigned int>(static_cast<unsigned int>((size.height-4)*0.1f),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-4)-slider->size.height)*value+2);
				slider->setSlideBar(this);
			}

			MouseDelegate mPressed;
			mPressed.bind(this,&SlideBar::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

		}

		SlideBar::SlideBar(float _minV,float _maxV,float _value,int _type):minV(_minV),maxV(_maxV),type(_type),value(0)
		{
			setValue(_value);
			if(type==Horizontal)
			{
				slider=new SlideBarSlider(SlideBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
				size.width=10;
				size.height=20;
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-4)*0.1f),4);
				slider->size.height=16;
				slider->position.x=static_cast<int>(((size.width-4)-slider->size.width)*value+2);
				slider->position.y=2;
				slider->setSlideBar(this);
			}
			else if(type==Vertical)
			{
				slider=new SlideBarSlider(SlideBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
				size.width=20;
				size.height=10;
				slider->size.width=16;
				slider->size.height=std::max<unsigned int>(static_cast<unsigned int>((size.height-4)*0.1f),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-4)-slider->size.height)*value+2);
				slider->setSlideBar(this);
			}

			MouseDelegate mPressed;
			mPressed.bind(this,&SlideBar::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

		void SlideBar::mousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
            Util::Position p(position);
            Util::Graphics::getSingleton().pushPosition(p);
			slider->paint();
			Util::Graphics::getSingleton().popPosition();
		}

		void SlideBar::pack()
		{
			if(type==Horizontal)
			{
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-4)*0.1f),4);
				slider->size.height=16;
				slider->position.x=static_cast<int>(((size.width-4)-slider->size.width)*value+2);
				slider->position.y=2;
			}
			else if(type==Vertical)
			{
				slider->size.width=16;
				slider->size.height=std::max<unsigned int>(static_cast<int>((size.height-4)*0.1f),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-4)-slider->size.height)*value+2);		
			}
		}

		SlideBar::~SlideBar(void)
		{
			delete slider;
		}
	}
}
