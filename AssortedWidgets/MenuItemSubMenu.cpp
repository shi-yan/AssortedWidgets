#include "MenuItemSubMenu.h"
#include "FontEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

        MenuItemSubMenu::MenuItemSubMenu(const std::string &_text)
            :m_text(_text),
              m_status(normal),
              m_expand(false),
              m_left(24),
              m_top(4),
              m_bottom(4),
              m_right(2)
		{
            m_size=getPreferedSize();

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseReleased));

            m_menuList.m_position.x=232-9;
            m_menuList.m_position.y=0;
		}

        MenuItemSubMenu::MenuItemSubMenu(const char *_text)
            :m_text(_text),
              m_status(normal),
              m_expand(false),
              m_left(24),
              m_top(4),
              m_bottom(4),
              m_right(2)
		{
            m_size=getPreferedSize();

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuItemSubMenu::mouseReleased));

            m_menuList.m_position.x=232-9;
            m_menuList.m_position.y=0;
		}

		MenuItemSubMenu::~MenuItemSubMenu(void)
		{
		}

		void MenuItemSubMenu::mouseReleased(const Event::MouseEvent &e)
		{
            if(m_expand)
			{
                m_parentMenuList->setShrink();
                m_expand=false;
			}
			else
			{
                m_parentMenuList->setExpand(this);
                m_expand=true;
			}
            m_status=hover;
		}

		void MenuItemSubMenu::listMousePressed(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_expand && m_menuList.isIn(mx,my))
			{
                Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                m_menuList.processMousePressed(event);
			}

            if(m_menuList.isExpand() && m_menuList.getExpandMenu())
			{
                Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_PRESSED,mx-m_menuList.m_position.x,my-m_menuList.m_position.y,0);
                m_menuList.getExpandMenu()->listMousePressed(event);
			}
		}

		void MenuItemSubMenu::listMouseReleased(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_expand && m_menuList.isIn(mx,my))
			{
                Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                m_menuList.processMouseReleased(event);
			}

            if(m_menuList.isExpand() && m_menuList.getExpandMenu())
			{
                Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_RELEASED,mx-m_menuList.m_position.x,my-m_menuList.m_position.y,0);
                m_menuList.getExpandMenu()->listMouseReleased(event);
			}
		}

		void MenuItemSubMenu::listMouseMotion(const Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;
            if(m_expand && m_menuList.isIn(mx,my))
			{
                if(m_menuList.m_isHover)
				{
                    Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
                    m_menuList.processMouseMoved(event);
				}
				else
				{
                    Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                    m_menuList.processMouseEntered(event);
				}
			}
			else
			{
                if(m_menuList.m_isHover)
				{
                    Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                    m_menuList.processMouseExited(event);
				}
			}

            if(m_menuList.isExpand() && m_menuList.getExpandMenu())
			{
                Event::MouseEvent event(&m_menuList,Event::MouseEvent::MOUSE_MOTION,mx-m_menuList.m_position.x,my-m_menuList.m_position.y,0);
                m_menuList.getExpandMenu()->listMouseMotion(event);
			}
		}

        void MenuItemSubMenu::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
		}

        void MenuItemSubMenu::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            m_status=hover;//
			/*if(parentMenuList->isExpand())
			{
				parentMenuList->setExpand(this);
				expand=true;
			}
			else
			{
				if(!expand)
				{
					status=hover;
				}
			}*/
		}

        void MenuItemSubMenu::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            m_status=normal;
		}
	}
}
