#include "ScrollPanel.h"
#include "ScrollBar.h"
#include "ThemeEngine.h"
#include "Graphics.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        ScrollPanel::ScrollPanel(void)
            :m_content(0),
              m_offsetX(0),
              m_offsetY(0),
              m_horizontalScrollStyle(Auto),
              m_verticalScrollStyle(Auto),
              m_offsetXMax(0),
              m_offsetYMax(0),
              m_horizontalBarShow(false),
              m_verticalBarShow(false)
		{
            m_horizontalBar=new ScrollBar(ScrollBar::Horizontal);
            m_verticalBar=new ScrollBar(ScrollBar::Vertical);
			setHorizontalStyle(Element::Stretch);
			setVerticalStyle(Element::Stretch);
            m_horizontalBar->setScrollPanel(this);
            m_verticalBar->setScrollPanel(this);

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(ScrollPanel::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ScrollPanel::mouseReleased));
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(ScrollPanel::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(ScrollPanel::mouseExited));
            mouseMovedHandlerList.push_back(MOUSE_DELEGATE(ScrollPanel::mouseMoved));

			pack();
		}

		void ScrollPanel::mouseEntered(const Event::MouseEvent &e)
		{
            m_isHover=true;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_verticalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                m_verticalBar->processMouseEntered(event);
				return;
			}
            else if(m_horizontalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                m_horizontalBar->processMouseEntered(event);
				return;			
			}
		}

		void ScrollPanel::onValueChanged(ScrollBar *scrollBar)
		{
            if(scrollBar==m_horizontalBar)
			{
                m_offsetX=static_cast<unsigned int>(m_offsetXMax*scrollBar->getValue());
                if(m_content)
				{
                    m_content->m_position.x=-static_cast<int>(m_offsetX);
				}
			}
            else if(scrollBar==m_verticalBar)
			{
                m_offsetY=static_cast<unsigned int>(m_offsetYMax*scrollBar->getValue());
                if(m_content)
				{
                    m_content->m_position.y=-static_cast<int>(m_offsetY);
				}
			}
		}

		void ScrollPanel::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_verticalBar->isIn(mx,my))
			{
                if(m_verticalBar->m_isHover)
				{
                    Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
                    m_verticalBar->processMouseMoved(event);
					return;
				}
				else
				{
                    Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_verticalBar->processMouseEntered(event);
					return;
				}
			}
			else
			{
                if(m_verticalBar->m_isHover)
				{
                    Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_verticalBar->processMouseExited(event);
					return;				
				}
			}

            if(m_horizontalBar->isIn(mx,my))
			{
                if(m_horizontalBar->m_isHover)
				{
                    Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
                    m_horizontalBar->processMouseMoved(event);
					return;			
				}
				else
				{
                    Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_horizontalBar->processMouseEntered(event);
					return;			
				}
			}	
			else
			{
                if(m_horizontalBar->m_isHover)
				{
                    Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_horizontalBar->processMouseExited(event);
					return;					
				}
			}
		}

		void ScrollPanel::mouseExited(const Event::MouseEvent &e)
		{
            m_isHover=false;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_verticalBar->m_isHover)
			{
                Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_verticalBar->processMouseExited(event);
				return;
			}
            else if(m_horizontalBar->m_isHover)
			{
                Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_horizontalBar->processMouseExited(event);
				return;			
			}	
		}

		void ScrollPanel::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_verticalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_verticalBar->processMouseReleased(event);
				return;
			}
            else if(m_horizontalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_horizontalBar->processMouseReleased(event);
				return;			
			}
		}

		void ScrollPanel::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_verticalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_verticalBar,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_verticalBar->processMousePressed(event);
				return;
			}
            else if(m_horizontalBar->isIn(mx,my))
			{
                Event::MouseEvent event(m_horizontalBar,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_horizontalBar->processMousePressed(event);
				return;			
			}
		}

		void ScrollPanel::pack()
		{
            m_scissorWidth=m_size.m_width-2;
            m_scissorHeight=m_size.m_height-2;
            if(m_content)
			{
                if(m_content->m_size.m_width>m_size.m_width-17 && m_horizontalScrollStyle==Auto)
				{
                    m_horizontalBarShow=true;
                    m_scissorWidth-=18;
                    m_horizontalBar->m_position.x=2;
                    m_horizontalBar->m_position.y=m_size.m_height-16;
                    m_horizontalBar->m_size.m_width=m_size.m_width-18;
                    m_horizontalBar->pack();
				}
				else
				{
                    m_horizontalBar->setValue(0);
                    m_horizontalBarShow=false;
				}

                if(m_content->m_size.m_height>m_size.m_height-17 && m_verticalScrollStyle==Auto)
				{
                    m_verticalBarShow=true;
                    m_scissorHeight-=18;
                    m_verticalBar->m_position.x=m_size.m_width-16;
                    m_verticalBar->m_position.y=2;
                    m_verticalBar->m_size.m_height=m_size.m_height-18;
                    m_verticalBar->pack();
				}
				else
				{
                    m_verticalBar->setValue(0);
                    m_verticalBarShow=false;
				}

                m_offsetXMax=std::max<unsigned int>(m_content->m_size.m_width-(m_size.m_width-17),0);
                m_offsetYMax=std::max<unsigned int>(m_content->m_size.m_height-(m_size.m_height-17),0);
                m_offsetX=static_cast<unsigned int>(m_offsetXMax*m_horizontalBar->getValue());
                m_content->m_position.x=-static_cast<int>(m_offsetX);
                m_offsetY=static_cast<int>(m_offsetYMax*m_verticalBar->getValue());
                m_content->m_position.y=-static_cast<int>(m_offsetY);
			}
		}

		void ScrollPanel::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintScrollPanel(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);

            if(m_horizontalBarShow)
			{
                m_horizontalBar->paint();
			}
            if(m_verticalBarShow)
			{
                m_verticalBar->paint();
			}
			Util::Position sPosition(2,2);
            Util::Size sArea(m_scissorWidth,m_scissorHeight);
			Theme::ThemeEngine::getSingleton().getTheme().scissorBegin(sPosition,sArea);
            if(m_content)
			{
                m_content->paint();
			}
			Theme::ThemeEngine::getSingleton().getTheme().scissorEnd();
			Util::Graphics::getSingleton().popPosition();
		}

		ScrollPanel::~ScrollPanel(void)
		{
            delete m_horizontalBar;
            delete m_verticalBar;
		}
	}
}
