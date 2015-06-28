#include "ScrollBar.h"
#include "ScrollBarButton.h"
#include "ScrollBarSlider.h"
#include "Graphics.h"
#include "ThemeEngine.h"
#include "ScrollPanel.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		ScrollBar::ScrollBar(int _type):type(_type),value(0)
		{
			if(type==Horizontal)
			{
				min=new ScrollBarButton(ScrollBarButton::HorizontalLeft);
				max=new ScrollBarButton(ScrollBarButton::HorizontalRight);
				slider=new ScrollBarSlider(ScrollBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
                m_size.width=40;
                m_size.height=15;
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-30)*0.1f),4);
                slider->m_size.height=11;
                slider->m_position.x=static_cast<int>(((m_size.width-34)-slider->m_size.width)*value+17);
                slider->m_position.y=2;
				slider->setScrollBar(this);
			}
			else if(type==Vertical)
			{
				min=new ScrollBarButton(ScrollBarButton::VerticalTop);
				max=new ScrollBarButton(ScrollBarButton::VerticalBottom);
				slider=new ScrollBarSlider(ScrollBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.width=15;
                m_size.height=40;
                slider->m_size.width=11;
                slider->m_size.height=std::max<unsigned int>(static_cast<unsigned int>((m_size.height-30)*0.1),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-34)-slider->m_size.height)*value+17);
				slider->setScrollBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseReleased));
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseExited));
            mouseMovedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseMoved));
            min->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::onMinReleased));
            max->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::onMaxReleased));
		}

		void ScrollBar::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(min->isIn(mx,my))
			{
				if(!min->isHover)
				{
					Event::MouseEvent event(min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					min->processMouseEntered(event);
				}
			}
			else
			{
				if(min->isHover)
				{
					Event::MouseEvent event(min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					min->processMouseExited(event);
				}
			}

			if(max->isIn(mx,my))
			{
				if(!max->isHover)
				{
					Event::MouseEvent event(max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					max->processMouseEntered(event);
				}
			}	
			else
			{
				if(max->isHover)
				{
					Event::MouseEvent event(max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					max->processMouseExited(event);
				}
			}
		}

		void ScrollBar::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
				min->processMouseEntered(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
				max->processMouseEntered(event);
				return;			
			}
		}

		void ScrollBar::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(min->isHover)
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
				min->processMouseExited(event);
				return;
			}
			else if(max->isHover)
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
				max->processMouseExited(event);
				return;			
			}
		}

		void ScrollBar::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				min->processMouseReleased(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				max->processMouseReleased(event);
				return;			
			}
		}

		void ScrollBar::onMinReleased(const Event::MouseEvent &e)
		{
			value=std::max<float>(value-0.1f,0.0f);
			if(type==Horizontal)
			{
                slider->m_position.x=static_cast<int>(((m_size.width-34)-slider->m_size.width)*value+17);
                slider->m_position.y=2;
			}
			else if(type==Vertical)
			{
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-34)-slider->m_size.height)*value+17);
			}
			onValueChanged();
		}

		void ScrollBar::onMaxReleased(const Event::MouseEvent &e)
		{
			value=std::min<float>(value+0.1f,1.0f);
			if(type==Horizontal)
			{
                slider->m_position.x=static_cast<int>(((m_size.width-34)-slider->m_size.width)*value+17);
                slider->m_position.y=2;
			}
			else if(type==Vertical)
			{
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-34)-slider->m_size.height)*value+17);
			}
			onValueChanged();
		}

		void ScrollBar::onValueChanged()
		{
			parent->onValueChanged(this);
		};

		void ScrollBar::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			if(slider->isIn(mx,my))
			{
				Event::MouseEvent event(slider,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
				slider->processMousePressed(event);
				return;
			}
			else if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				min->processMousePressed(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				max->processMousePressed(event);
				return;			
			}
		}

		void ScrollBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintScrollBar(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
			min->paint();
			max->paint();
			slider->paint();
			Util::Graphics::getSingleton().popPosition();
		};

		void ScrollBar::pack()
		{
			if(type==Horizontal)
			{
                min->m_position.x=0;
                min->m_position.y=0;
                max->m_position.x=m_size.width-15;
                max->m_position.y=0;
                slider->m_size.width=std::max<unsigned int>(static_cast<unsigned int>((m_size.width-30)*0.1f),4);
                slider->m_size.height=11;
                slider->m_position.x=static_cast<int>(((m_size.width-34)-slider->m_size.width)*value+17);
                slider->m_position.y=2;
			}
			else if(type==Vertical)
			{
                min->m_position.x=0;
                min->m_position.y=0;
                max->m_position.x=0;
                max->m_position.y=m_size.height-15;
                slider->m_size.width=11;
                slider->m_size.height=std::max<unsigned int>(static_cast<unsigned int>((m_size.height-30)*0.1f),4);
                slider->m_position.x=2;
                slider->m_position.y=static_cast<int>(((m_size.height-34)-slider->m_size.height)*value+17);
			}
		};

		ScrollBar::~ScrollBar(void)
		{
			delete slider;
			delete min;
			delete max;
		}
	}
}
