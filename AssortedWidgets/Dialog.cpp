#include "Dialog.h"
#include "DialogManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		Dialog::Dialog(std::string &tittle,int x,int y,unsigned int width,unsigned int height):tittleBar(tittle),top(12),bottom(14),left(12),right(12),borderUpLeft(9,7,4,4),borderUpRight(width-9-4,7,4,4),borderUp(13,7,width-26,4),borderLeft(9,11,4,height-27),borderRight(width-13,11,4,height-27),borderBottomLeft(9,height-27,4,4),borderBottom(13,height-27,width-26,4),borderBottomRight(width-13,height-27,4,4),dragable(true),resizable(true),showType(None)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
			tittleBar.setDialog(this);
			borderUpLeft.setParent(this);
			borderUpRight.setParent(this);
			borderUp.setParent(this);
			borderLeft.setParent(this);
			borderRight.setParent(this);
			borderBottomLeft.setParent(this);
			borderBottom.setParent(this);
			borderBottomRight.setParent(this);
			MouseDelegate mPressed;
			mPressed.bind(this,&Dialog::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&Dialog::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mEntered;
			mEntered.bind(this,&Dialog::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&Dialog::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			pack();
		}

		Dialog::Dialog(char *tittle,int x,int y,unsigned int width,unsigned int height):tittleBar(tittle),top(12),bottom(14),left(12),right(12),borderUpLeft(9,7,4,4),borderUpRight(width-9-4,7,4,4),borderUp(13,7,width-26,4),borderLeft(9,11,4,height-27),borderRight(width-13,11,4,height-27),borderBottomLeft(9,height-27,4,4),borderBottom(13,height-27,width-26,4),borderBottomRight(width-13,height-27,4,4),dragable(true),resizable(true),showType(None)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
			tittleBar.setDialog(this);
			borderUpLeft.setParent(this);
			borderUpRight.setParent(this);
			borderUp.setParent(this);
			borderLeft.setParent(this);
			borderRight.setParent(this);
			borderBottomLeft.setParent(this);
			borderBottom.setParent(this);
			borderBottomRight.setParent(this);
			MouseDelegate mPressed;
			mPressed.bind(this,&Dialog::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&Dialog::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mEntered;
			mEntered.bind(this,&Dialog::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&Dialog::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mMoved;
			mMoved.bind(this,&Dialog::mouseMoved);
			mouseMovedHandlerList.push_back(mMoved);
		
			pack();
		}

		void Dialog::Close()
		{
			if(showType==Modal)
			{
				Manager::DialogManager::getSingleton().dropModalDialog();
			}
			else if(showType==Modeless)
			{
				Manager::DialogManager::getSingleton().dropModelessDialog(this);
			}
		};

		void Dialog::mouseEntered(const Event::MouseEvent &e)
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

		void Dialog::mouseExited(const Event::MouseEvent &e)
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

		void Dialog::mouseMoved(const Event::MouseEvent &e)
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

		void Dialog::mousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(dragable)
			{
				if(tittleBar.isIn(mx,my))
				{
					Event::MouseEvent event(&tittleBar,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					tittleBar.processMousePressed(event);
					return;
				}
			}
			if(resizable)
			{
				if(borderUpLeft.isIn(mx,my))
				{
					Event::MouseEvent event(&borderUpLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderUpLeft.processMousePressed(event);
					return;				
				}
				else if(borderUpRight.isIn(mx,my))
				{
					Event::MouseEvent event(&borderUpRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderUpRight.processMousePressed(event);
					return;							
				}
				else if(borderUp.isIn(mx,my))
				{
					Event::MouseEvent event(&borderUp,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderUp.processMousePressed(event);
					return;										
				}
				else if(borderLeft.isIn(mx,my))
				{
					Event::MouseEvent event(&borderLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderLeft.processMousePressed(event);
					return;													
				}
				else if(borderRight.isIn(mx,my))
				{
					Event::MouseEvent event(&borderRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderRight.processMousePressed(event);
					return;													
				}
				else if(borderBottomLeft.isIn(mx,my))
				{
					Event::MouseEvent event(&borderBottomLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderBottomLeft.processMousePressed(event);
					return;													
				}
				else if(borderBottom.isIn(mx,my))
				{
					Event::MouseEvent event(&borderBottom,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderBottom.processMousePressed(event);
					return;													
				}
				else if(borderBottomRight.isIn(mx,my))
				{
					Event::MouseEvent event(&borderBottomRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					borderBottomRight.processMousePressed(event);
					return;													
				}
			}
			
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

		void Dialog::mouseReleased(const Event::MouseEvent &e)
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

		void Dialog::pack()
		{
			tittleBar.position.x=left;
			tittleBar.position.y=top;
			tittleBar.size.width=size.width-left-right;
			tittleBar.size.height=20;

			borderUpRight.position.x=size.width-13;
			borderUp.size.width=size.width-26;
			borderLeft.size.height=size.height-27;
			borderRight.position.x=size.width-13;
			borderRight.size.height=size.height-27;

			borderBottomLeft.position.y=size.height-15;
			
			borderBottom.position.y=size.height-15;
			borderBottom.size.width=size.width-26;
			
			borderBottomRight.position.x=size.width-13;
			borderBottomRight.position.y=size.height-15;

			contentPosition=Util::Position(left,(top+tittleBar.size.height+2));
			contentSize=Util::Size(size.width-left-right,size.height-top-bottom-2-tittleBar.size.height);


			//contentSize=Util::Size(100,100);

			if(layout)
			{
				
				layout->updateLayout(childList,contentPosition,contentSize);
			}
		};

		Dialog::~Dialog(void)
		{
		}
	}
}