#include "Dialog.h"
#include "DialogManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        Dialog::Dialog(std::string &tittle,int x,int y,unsigned int width,unsigned int height)
            :m_tittleBar(tittle),
              m_top(12),
              m_bottom(14),
              m_left(12),
              m_right(12),
              m_borderUpLeft(9,7,4,4),
              m_borderUpRight(width-9-4,7,4,4),
              m_borderUp(13,7,width-26,4),
              m_borderLeft(9,11,4,height-27),
              m_borderRight(width-13,11,4,height-27),
              m_borderBottomLeft(9,height-27,4,4),
              m_borderBottom(13,height-27,width-26,4),
              m_borderBottomRight(width-13,height-27,4,4),
              m_dragable(true),
              m_resizable(true),
              m_showType(None)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
            m_tittleBar.setDialog(this);
            m_borderUpLeft.setParent(this);
            m_borderUpRight.setParent(this);
            m_borderUp.setParent(this);
            m_borderLeft.setParent(this);
            m_borderRight.setParent(this);
            m_borderBottomLeft.setParent(this);
            m_borderBottom.setParent(this);
            m_borderBottomRight.setParent(this);
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

        Dialog::Dialog(char *tittle,int x,int y,unsigned int width,unsigned int height)
            :m_tittleBar(tittle),
              m_top(12),
              m_bottom(14),
              m_left(12),
              m_right(12),
              m_borderUpLeft(9,7,4,4),
              m_borderUpRight(width-9-4,7,4,4),
              m_borderUp(13,7,width-26,4),
              m_borderLeft(9,11,4,height-27),
              m_borderRight(width-13,11,4,height-27),
              m_borderBottomLeft(9,height-27,4,4),
              m_borderBottom(13,height-27,width-26,4),
              m_borderBottomRight(width-13,height-27,4,4),
              m_dragable(true),
              m_resizable(true),
              m_showType(None)
		{
			position.x=x;
			position.y=y;
			size.width=width;
			size.height=height;
            m_tittleBar.setDialog(this);
            m_borderUpLeft.setParent(this);
            m_borderUpRight.setParent(this);
            m_borderUp.setParent(this);
            m_borderLeft.setParent(this);
            m_borderRight.setParent(this);
            m_borderBottomLeft.setParent(this);
            m_borderBottom.setParent(this);
            m_borderBottomRight.setParent(this);
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
            if(m_showType==Modal)
			{
				Manager::DialogManager::getSingleton().dropModalDialog();
			}
            else if(m_showType==Modeless)
			{
				Manager::DialogManager::getSingleton().dropModelessDialog(this);
			}
        }

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
            if(m_dragable)
			{
                if(m_tittleBar.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_tittleBar,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_tittleBar.processMousePressed(event);
					return;
				}
			}
            if(m_resizable)
			{
                if(m_borderUpLeft.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderUpLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderUpLeft.processMousePressed(event);
					return;				
				}
                else if(m_borderUpRight.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderUpRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderUpRight.processMousePressed(event);
					return;							
				}
                else if(m_borderUp.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderUp,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderUp.processMousePressed(event);
					return;										
				}
                else if(m_borderLeft.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderLeft.processMousePressed(event);
					return;													
				}
                else if(m_borderRight.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderRight.processMousePressed(event);
					return;													
				}
                else if(m_borderBottomLeft.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderBottomLeft,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderBottomLeft.processMousePressed(event);
					return;													
				}
                else if(m_borderBottom.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderBottom,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderBottom.processMousePressed(event);
					return;													
				}
                else if(m_borderBottomRight.isIn(mx,my))
				{
                    Event::MouseEvent event(&m_borderBottomRight,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_borderBottomRight.processMousePressed(event);
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
            m_tittleBar.position.x=m_left;
            m_tittleBar.position.y=m_top;
            m_tittleBar.size.width=size.width-m_left-m_right;
            m_tittleBar.size.height=20;

            m_borderUpRight.position.x=size.width-13;
            m_borderUp.size.width=size.width-26;
            m_borderLeft.size.height=size.height-27;
            m_borderRight.position.x=size.width-13;
            m_borderRight.size.height=size.height-27;

            m_borderBottomLeft.position.y=size.height-15;
			
            m_borderBottom.position.y=size.height-15;
            m_borderBottom.size.width=size.width-26;
			
            m_borderBottomRight.position.x=size.width-13;
            m_borderBottomRight.position.y=size.height-15;

            m_contentPosition=Util::Position(m_left,(m_top+m_tittleBar.size.height+2));
            m_contentSize=Util::Size(size.width-m_left-m_right,size.height-m_top-m_bottom-2-m_tittleBar.size.height);


			//contentSize=Util::Size(100,100);

			if(layout)
			{
				
                layout->updateLayout(childList,m_contentPosition,m_contentSize);
			}
        }

		Dialog::~Dialog(void)
		{
		}
	}
}
