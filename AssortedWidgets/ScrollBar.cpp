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
        ScrollBar::ScrollBar(int _type)
            :m_type(_type)
            ,m_value(0)
		{
            if(m_type==Horizontal)
			{
                m_min=new ScrollBarButton(ScrollBarButton::HorizontalLeft);
                m_max=new ScrollBarButton(ScrollBarButton::HorizontalRight);
                m_slider=new ScrollBarSlider(ScrollBarSlider::Horizontal);
                setHorizontalStyle(Element::Stretch);
                setVerticalStyle(Element::Fit);
                m_size.m_width=40;
                m_size.m_height=15;
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-30)*0.1f),4);
                m_slider->m_size.m_height=11;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-34)-m_slider->m_size.m_width)*m_value+17);
                m_slider->m_position.y=2;
                m_slider->setScrollBar(this);
			}
            else if(m_type==Vertical)
			{
                m_min=new ScrollBarButton(ScrollBarButton::VerticalTop);
                m_max=new ScrollBarButton(ScrollBarButton::VerticalBottom);
                m_slider=new ScrollBarSlider(ScrollBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
                m_size.m_width=15;
                m_size.m_height=40;
                m_slider->m_size.m_width=11;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_height-30)*0.1),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-34)-m_slider->m_size.m_height)*m_value+17);
                m_slider->setScrollBar(this);
			}

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseReleased));
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseExited));
            mouseMovedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::mouseMoved));
            m_min->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::onMinReleased));
            m_max->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollBar::onMaxReleased));
		}

		void ScrollBar::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_min->isIn(mx,my))
			{
                if(!m_min->m_isHover)
				{
                    Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_min->processMouseEntered(event);
				}
			}
			else
			{
                if(m_min->m_isHover)
				{
                    Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_min->processMouseExited(event);
				}
			}

            if(m_max->isIn(mx,my))
			{
                if(!m_max->m_isHover)
				{
                    Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_max->processMouseEntered(event);
				}
			}	
			else
			{
                if(m_max->m_isHover)
				{
                    Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_max->processMouseExited(event);
				}
			}
		}

		void ScrollBar::mouseEntered(const Event::MouseEvent &e)
		{
            m_isHover=true;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_min->isIn(mx,my))
			{
                Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                m_min->processMouseEntered(event);
				return;
			}
            else if(m_max->isIn(mx,my))
			{
                Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                m_max->processMouseEntered(event);
				return;			
			}
		}

		void ScrollBar::mouseExited(const Event::MouseEvent &e)
		{
            m_isHover=false;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_min->m_isHover)
			{
                Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_min->processMouseExited(event);
				return;
			}
            else if(m_max->m_isHover)
			{
                Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_max->processMouseExited(event);
				return;			
			}
		}

		void ScrollBar::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_min->isIn(mx,my))
			{
                Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_min->processMouseReleased(event);
				return;
			}
            else if(m_max->isIn(mx,my))
			{
                Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_max->processMouseReleased(event);
				return;			
			}
		}

        void ScrollBar::onMinReleased(const Event::MouseEvent &)
		{
            m_value=std::max<float>(m_value-0.1f,0.0f);
            if(m_type==Horizontal)
			{
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-34)-m_slider->m_size.m_width)*m_value+17);
                m_slider->m_position.y=2;
			}
            else if(m_type==Vertical)
			{
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-34)-m_slider->m_size.m_height)*m_value+17);
			}
			onValueChanged();
		}

        void ScrollBar::onMaxReleased(const Event::MouseEvent &)
		{
            m_value=std::min<float>(m_value+0.1f,1.0f);
            if(m_type==Horizontal)
			{
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-34)-m_slider->m_size.m_width)*m_value+17);
                m_slider->m_position.y=2;
			}
            else if(m_type==Vertical)
			{
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-34)-m_slider->m_size.m_height)*m_value+17);
			}
			onValueChanged();
		}

		void ScrollBar::onValueChanged()
		{
            m_parent->onValueChanged(this);
        }

		void ScrollBar::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_slider->isIn(mx,my))
			{
                Event::MouseEvent event(m_slider,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_slider->processMousePressed(event);
				return;
			}
            else if(m_min->isIn(mx,my))
			{
                Event::MouseEvent event(m_min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_min->processMousePressed(event);
				return;
			}
            else if(m_max->isIn(mx,my))
			{
                Event::MouseEvent event(m_max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_max->processMousePressed(event);
				return;			
			}
		}

		void ScrollBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintScrollBar(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
            m_min->paint();
            m_max->paint();
            m_slider->paint();
			Util::Graphics::getSingleton().popPosition();
        }

		void ScrollBar::pack()
		{
            if(m_type==Horizontal)
			{
                m_min->m_position.x=0;
                m_min->m_position.y=0;
                m_max->m_position.x=m_size.m_width-15;
                m_max->m_position.y=0;
                m_slider->m_size.m_width=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_width-30)*0.1f),4);
                m_slider->m_size.m_height=11;
                m_slider->m_position.x=static_cast<int>(((m_size.m_width-34)-m_slider->m_size.m_width)*m_value+17);
                m_slider->m_position.y=2;
			}
            else if(m_type==Vertical)
			{
                m_min->m_position.x=0;
                m_min->m_position.y=0;
                m_max->m_position.x=0;
                m_max->m_position.y=m_size.m_height-15;
                m_slider->m_size.m_width=11;
                m_slider->m_size.m_height=std::max<unsigned int>(static_cast<unsigned int>((m_size.m_height-30)*0.1f),4);
                m_slider->m_position.x=2;
                m_slider->m_position.y=static_cast<int>(((m_size.m_height-34)-m_slider->m_size.m_height)*m_value+17);
			}
        }

		ScrollBar::~ScrollBar(void)
		{
            delete m_slider;
            delete m_min;
            delete m_max;
		}
	}
}
