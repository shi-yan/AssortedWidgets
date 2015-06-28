#include "DropList.h"
#include "ThemeEngine.h"
#include "Graphics.h"
#include "DropListManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        DropList::DropList(void)
            :m_selectedItem(0),
              m_spacer(2),
              m_left(4),
              m_right(4),
              m_top(4),
              m_bottom(4),
              m_dropped(false)
		{
            m_size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
            m_button.m_position.x=m_size.width-18;
            m_button.m_position.y=2;

            mousePressedHandlerList.push_back(MOUSE_DELEGATE(DropList::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(DropList::mouseReleased));
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(DropList::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(DropList::mouseExited));
            mouseMovedHandlerList.push_back(MOUSE_DELEGATE(DropList::mouseMoved));
            m_button.mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(DropList::onDropReleased));
		
		}

		void DropList::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintDropList(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
            m_button.paint();
			Util::Graphics::getSingleton().popPosition();

		}

		void DropList::onDropReleased(const Event::MouseEvent &e)
		{
            if(m_dropped)
			{
				Manager::DropListManager::getSingleton().shrinkBack();
                m_dropped=false;
			}
			else
			{
				Manager::DropListManager::getSingleton().setDropped(this,e.getX(),e.getY());
                m_dropped=true;
			}
		}

		void DropList::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_button.isIn(mx,my))
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_button.processMousePressed(event);
				return;
			}
		}

		void DropList::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_button.isIn(mx,my))
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_button.processMouseReleased(event);
				return;
			}
		}

		void DropList::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_button.isIn(mx,my))
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                m_button.processMouseEntered(event);
				return;
			}
		}

		void DropList::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_button.isHover)
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_button.processMouseExited(event);
				return;
			}
		}

		void DropList::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_button.isIn(mx,my))
			{
                if(!m_button.isHover)
				{
                    Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_button.processMouseEntered(event);
				}
			}
			else
			{
                if(m_button.isHover)
				{
                    Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_button.processMouseExited(event);
				}
			}
		}

		void DropList::pack()
		{
            m_button.m_position.x=m_size.width-18;
            m_button.m_position.y=2;
		}

		DropList::~DropList(void)
		{
		}
	}
}
