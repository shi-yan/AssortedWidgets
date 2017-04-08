#include "Panel.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        Panel::Panel(void)
            : m_top(2),
              m_bottom(2),
              m_left(2),
              m_right(2),
              m_contentPosition(),
              m_contentSize()
        {
            m_position.x=0;
            m_position.y=0;
            m_size.m_width=50;
            m_size.m_height=50;
			setHorizontalStyle(Element::Stretch);
			setVerticalStyle(Element::Stretch);

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(Panel::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(Panel::mouseReleased));
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(Panel::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(Panel::mouseExited));

			pack();
		}

		Panel::~Panel(void)
		{
		}

        void Panel::mouseEntered(const Event::MouseEvent &e)
		{
            m_isHover=true;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					(*iter)->processMouseEntered(event);
					break;
				}
			}
		}

		void Panel::mouseExited(const Event::MouseEvent &e)
		{
            m_isHover=false;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
                if((*iter)->m_isHover)
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					(*iter)->processMouseExited(event);
					break;
				}
			}
		}

		void Panel::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
                    if((*iter)->m_isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_MOTION,mx,my,0);
						(*iter)->processMouseMoved(event);
						break;
					}
					else
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
						(*iter)->processMouseEntered(event);
						break;						
					}
				}
				else
				{
                    if((*iter)->m_isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						(*iter)->processMouseExited(event);
						break;
					}
				}
			}		
		}

		void Panel::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					(*iter)->processMousePressed(event);
					break;
				}
			}
		}

		void Panel::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
					(*iter)->processMouseReleased(event);
					break;
				}
			}
		}

		void Panel::pack()
		{
            m_contentPosition=Util::Position(m_left, m_top);
            m_contentSize=Util::Size(m_size.m_width-m_left-m_right,m_size.m_height-m_top-m_bottom);

			if(layout)
			{
                layout->updateLayout(childList,m_contentPosition,m_contentSize);
			}
        }
	}
}
