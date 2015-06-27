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
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
            m_button.position.x=size.width-18;
            m_button.position.y=2;

			MouseDelegate mPressed;
			mPressed.bind(this,&DropList::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&DropList::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mEntered;
			mEntered.bind(this,&DropList::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&DropList::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mMoved;
			mMoved.bind(this,&DropList::mouseMoved);
			mouseMovedHandlerList.push_back(mMoved);

			MouseDelegate dropReleased;
			dropReleased.bind(this,&DropList::onDropReleased);
            m_button.mouseReleasedHandlerList.push_back(dropReleased);
		
		}

		void DropList::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintDropList(this);
            Util::Position p(position);
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
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
            if(m_button.isIn(mx,my))
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_button.processMousePressed(event);
				return;
			}
		}

		void DropList::mouseReleased(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
            if(m_button.isHover)
			{
                Event::MouseEvent event(&m_button,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                m_button.processMouseExited(event);
				return;
			}
		}

		void DropList::mouseMoved(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
            m_button.position.x=size.width-18;
            m_button.position.y=2;
		}

		DropList::~DropList(void)
		{
		}
	}
}
