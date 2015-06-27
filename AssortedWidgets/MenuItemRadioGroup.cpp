#include "MenuItemRadioGroup.h"
#include "MenuItemRadioButton.h"
#include "MouseEvent.h"
#include <cmath>

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuItemRadioGroup::MenuItemRadioGroup(void)
            :m_left(0),
              m_right(0),
              m_top(0),
              m_bottom(4),
              m_spacer(2),
              m_minimizeSize(232),
              m_currentSelection(0)
		{
			MouseDelegate mEntered;
			mEntered.bind(this,&MenuItemRadioGroup::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemRadioGroup::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemRadioGroup::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemRadioGroup::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mMoved;
			mMoved.bind(this,&MenuItemRadioGroup::mouseMoved);
			mouseMovedHandlerList.push_back(mMoved);

		}

		MenuItemRadioGroup::~MenuItemRadioGroup(void)
		{
		}

		void MenuItemRadioGroup::mousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			std::vector<MenuItemRadioButton*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					(*iter)->processMousePressed(event);
				}
			}
		}
		
		void MenuItemRadioGroup::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			mouseMoved(e);
		}

		void MenuItemRadioGroup::mouseReleased(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			std::vector<MenuItemRadioButton*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
					(*iter)->processMouseReleased(event);
				}
			}
		}

		void MenuItemRadioGroup::mouseMoved(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			std::vector<MenuItemRadioButton*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					if((*iter)->isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_MOTION,mx,my,0);
						(*iter)->processMouseMoved(event);
					}
					else
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
						(*iter)->processMouseEntered(event);
					}
				}
				else
				{
					if((*iter)->isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						(*iter)->processMouseExited(event);
					}
				}
			}
		}

		void MenuItemRadioGroup::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			mouseMoved(e);
		}


		void MenuItemRadioGroup::addItem(MenuItemRadioButton *item)
		{
            m_itemList.push_back(item);
			item->setGroup(this);
			updateLayout();
		};

		void MenuItemRadioGroup::updateLayout()
		{
            unsigned int tempX=m_left;
            unsigned int tempY=m_top;
            size.width=m_minimizeSize;
			size.height=0;
			std::vector<MenuItemRadioButton*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				Util::Size itemSize=(*iter)->getPreferedSize();
                size.width=std::max(size.width,itemSize.width);
                size.height+=itemSize.height+m_spacer;
				(*iter)->position.x=tempX;
				(*iter)->position.y=tempY;
                tempY+=m_spacer+itemSize.height;
			}
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				(*iter)->size.width=size.width;
			}
            size.width+=m_left+m_right;
            size.height+=m_top+m_bottom-m_spacer;
		}

		void MenuItemRadioGroup::paint()
		{
            Util::Position p(position);
            Util::Graphics::getSingleton().pushPosition(p);
			std::vector<MenuItemRadioButton *>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				(*iter)->paint();
			}
			Util::Graphics::getSingleton().popPosition();
		}

		void MenuItemRadioGroup::setSelection(size_t index)
		{
            if(m_currentSelection)
			{
                m_currentSelection->off();
			}
            m_currentSelection=m_itemList[index];
        }
		int MenuItemRadioGroup::getSelection()
		{
            for(size_t i=0;i<m_itemList.size();++i)
			{
                if(m_itemList[i]==m_currentSelection)
				{
					return static_cast<int>(i);
				}
			}
			return -1;
		};
		void MenuItemRadioGroup::setSelection(MenuItemRadioButton *item)
		{
            if(m_currentSelection)
			{
                m_currentSelection->off();
			}
            m_currentSelection=item;
		}
	}
}
