#include "Menu.h"
#include "MenuBar.h"
#include "FontEngine.h"
#include "MenuItemSubMenu.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        Menu::Menu(const std::string &_text)
            : m_expand(false),
              m_text(_text),
              m_status(normal),
              m_menuList(),
              m_menuBar(0)
		{
            m_size=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(m_text);
            m_size.m_width+=12;
            m_size.m_height=20;
            m_position.x=100;
            m_position.y=100;

            mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(Menu::mouseEntered));
            mouseExitedHandlerList.push_back(MOUSE_DELEGATE(Menu::mouseExited));
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(Menu::mousePressed));
            mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(Menu::mouseReleased));

            m_menuList.m_position.x=-9;
            m_menuList.m_position.y=25;
		}

		void Menu::mouseReleased(const Event::MouseEvent &e)
		{
            (void) e;
            m_status=hover;
            if(m_expand)
			{
                m_menuBar->setShrink();
                m_expand=false;
			}
			else
			{
                m_menuBar->setExpand(this);
                m_expand=true;
			}
		}

		void Menu::listMousePressed(const Event::MouseEvent &e)
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

		void Menu::listMouseReleased(const Event::MouseEvent &e)
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

		void Menu::listMouseMotion(const Event::MouseEvent &e)
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

        void Menu::mousePressed(const Event::MouseEvent &)
		{
            m_status=pressed;
		}

        void Menu::mouseEntered(const Event::MouseEvent &)
		{
            m_isHover=true;
            if(m_menuBar->isExpand())
			{
                m_menuBar->setExpand(this);
                m_expand=true;
			}
			else
			{
                if(!m_expand)
				{
                    m_status=hover;
				}
			}
        }

        void Menu::mouseExited(const Event::MouseEvent &)
		{
            m_isHover=false;
            if(!m_expand)
			{
                m_status=normal;
			}
        }

		Menu::~Menu(void)
		{
		}
	}
}
