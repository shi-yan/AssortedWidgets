#include "Panel.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		Panel::Panel(void):left(2),right(2),top(2),bottom(2)
		{
			position.x=0;
			position.y=0;
			size.width=50;
			size.height=50;
			setHorizontalStyle(Element::Stretch);
			setVerticalStyle(Element::Stretch);

			MouseDelegate mPressed;
			mPressed.bind(this,&Panel::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&Panel::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mEntered;
			mEntered.bind(this,&Panel::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&Panel::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			pack();
		}

		Panel::~Panel(void)
		{
		}


				void Panel::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
			isHover=false;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isHover)
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					(*iter)->processMouseExited(event);
					break;
				}
			}
		}

		void Panel::mouseMoved(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			std::vector<Element*>::iterator iter;
			for(iter=childList.begin();iter<childList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					if((*iter)->isHover)
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
					if((*iter)->isHover)
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
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			
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
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
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
			contentPosition=Util::Position(left,top);
			contentSize=Util::Size(size.width-left-right,size.height-top-bottom);

			if(layout)
			{
				layout->updateLayout(childList,contentPosition,contentSize);
			}
		};
	}
}