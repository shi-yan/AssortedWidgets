#include "MenuList.h"
#include "MenuItem.h"
#include "MouseEvent.h"
#include "MenuItemSubMenu.h"
#include <cmath>

namespace AssortedWidgets
{
	namespace Widgets
	{
        MenuList::MenuList(void)
            :m_itemList(),
              m_minimizeSize(232),
              m_top(6),
              m_bottom(16),
              m_left(9),
              m_right(9),
              m_spacer(2),
              m_expandSubMenu(nullptr),
              m_expand(false)
        {
            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuList::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuList::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuList::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuList::mouseReleased));
            mouseMovedHandlerList.push_back(MOUSE_DELEGATE(MenuList::mouseMoved));
		}

		MenuList::~MenuList(void)
		{
		}
			
		void MenuList::addItem(MenuItem *item)
		{
            m_itemList.push_back(item);
			item->setMenuList(this);
			updateLayout();
		}

		void MenuList::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintMenuList(this);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
			std::vector<MenuItem *>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				(*iter)->paint();
			}
			Util::Graphics::getSingleton().popPosition();
		}

		void MenuList::updateLayout()
		{
            unsigned int tempX=m_left;
            unsigned int tempY=m_top;
            m_size.m_width=m_minimizeSize;
            m_size.m_height=0;
			std::vector<MenuItem*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				Util::Size itemSize=(*iter)->getPreferedSize();
                m_size.m_width=std::max(m_size.m_width,itemSize.m_width);
                m_size.m_height+=itemSize.m_height+m_spacer;
                (*iter)->m_position.x=tempX;
                (*iter)->m_position.y=tempY;
                tempY+=m_spacer+itemSize.m_height;
			}
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
                (*iter)->m_size.m_width=m_size.m_width;
			}
            m_size.m_width+=m_left+m_right;
            m_size.m_height+=m_top+m_bottom-m_spacer;
		}

		void MenuList::mousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<MenuItem*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					(*iter)->processMousePressed(event);
				}
			}
		}
		
		void MenuList::mouseEntered(const Event::MouseEvent &e)
		{
            m_isHover=true;
			mouseMoved(e);
		}

		void MenuList::mouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<MenuItem*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
					(*iter)->processMouseReleased(event);
				}
			}
		}

		void MenuList::mouseMoved(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
			std::vector<MenuItem*>::iterator iter;
            for(iter=m_itemList.begin();iter<m_itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
                    if((*iter)->m_isHover)
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
                    if((*iter)->m_isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						(*iter)->processMouseExited(event);
					}
				}
			}
		}

		void MenuList::mouseExited(const Event::MouseEvent &e)
		{
            m_isHover=false;
			mouseMoved(e);
		}

		void MenuList::setExpand(MenuItemSubMenu *_expandSubMenu)
		{
            if(m_expandSubMenu)
			{
                m_expandSubMenu->shrink();
			}
            m_expandSubMenu=_expandSubMenu;
            m_expand=true;
		}

		void MenuList::setShrink()
		{
            if(m_expandSubMenu)
			{
                m_expandSubMenu->shrink();
			}
            m_expandSubMenu=0;
            m_expand=false;
		};
	}
}
