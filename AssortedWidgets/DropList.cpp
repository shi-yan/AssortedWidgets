#include "DropList.h"
#include "ThemeEngine.h"
#include "Graphics.h"
#include "DropListManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DropList::DropList(void):selectedItem(0),spacer(2),left(4),right(4),top(4),bottom(4),dropped(false)
		{
			size=getPreferedSize();
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			button.position.x=size.width-18;
			button.position.y=2;

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
			button.mouseReleasedHandlerList.push_back(dropReleased);
		
		}

		void DropList::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintDropList(this);
            Util::Position p(position);
            Util::Graphics::getSingleton().pushPosition(p);
			button.paint();
			Util::Graphics::getSingleton().popPosition();

		}

		void DropList::onDropReleased(const Event::MouseEvent &e)
		{
			if(dropped)
			{
				Manager::DropListManager::getSingleton().shrinkBack();
				dropped=false;
			}
			else
			{
				Manager::DropListManager::getSingleton().setDropped(this,e.getX(),e.getY());
				dropped=true;
			}
		}

		void DropList::mousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(button.isIn(mx,my))
			{
				Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
				button.processMousePressed(event);
				return;
			}
		}

		void DropList::mouseReleased(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(button.isIn(mx,my))
			{
				Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				button.processMouseReleased(event);
				return;
			}
		}

		void DropList::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(button.isIn(mx,my))
			{
				Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
				button.processMouseEntered(event);
				return;
			}
		}

		void DropList::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(button.isHover)
			{
				Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
				button.processMouseExited(event);
				return;
			}
		}

		void DropList::mouseMoved(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(button.isIn(mx,my))
			{
				if(!button.isHover)
				{
					Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					button.processMouseEntered(event);
				}
			}
			else
			{
				if(button.isHover)
				{
					Event::MouseEvent event(&button,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					button.processMouseExited(event);
				}
			}
		}

		void DropList::pack()
		{
			button.position.x=size.width-18;
			button.position.y=2;			
		}

		DropList::~DropList(void)
		{
		}
	}
}
