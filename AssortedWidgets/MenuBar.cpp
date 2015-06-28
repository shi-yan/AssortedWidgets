#include "MenuBar.h"
#include "Menu.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		MenuBar::~MenuBar(void)
		{
		}

		void MenuBar::setExpand(Menu *_expandMenu)
		{
            if(m_expandMenu)
			{
                m_expandMenu->shrink();
			}
            m_expandMenu=_expandMenu;
            m_expand=true;
        }

		void MenuBar::setShrink()
		{
            if(m_expandMenu)
			{
                m_expandMenu->shrink();
			}
            m_expandMenu=0;
            m_expand=false;
        }

		void MenuBar::addMenu(Menu *menu)
		{
            m_menuList.push_back(menu);
            menu->setMenuBar(this);
			updateLayout();
        }

		void MenuBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintMenuBar(this);
			std::vector<Menu*>::iterator iter;
            for(iter=m_menuList.begin();iter<m_menuList.end();++iter)
			{
				(*iter)->paint();
			}
        }

		void MenuBar::onMouseEnter(const Event::MouseEvent &e)
		{
            m_isHover=true;
			onMouseMove(e);
        }

		void MenuBar::onMouseExit(const Event::MouseEvent &e)
		{
            m_isHover=false;
			onMouseMove(e);
        }

		void MenuBar::onMousePressed(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
            for(iter=m_menuList.begin();iter<m_menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_PRESSED,e.getX(),e.getY(),0);
					(*iter)->processMousePressed(event);
				}
			}

            if(isExpand() && m_expandMenu)
			{
                m_expandMenu->listMousePressed(e);
			}
        }

		void MenuBar::onMouseReleased(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
            for(iter=m_menuList.begin();iter<m_menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_RELEASED,e.getX(),e.getY(),0);
					(*iter)->processMouseReleased(event);
				}
			}
            if(isExpand() && m_expandMenu)
			{
                m_expandMenu->listMouseReleased(e);
			}
        }

		void MenuBar::onMouseMove(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
            for(iter=m_menuList.begin();iter<m_menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
                    if(!(*iter)->m_isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,e.getX(),e.getY(),0);
						(*iter)->processMouseEntered(event);
					}
				}
				else
				{
                    if((*iter)->m_isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,e.getX(),e.getY(),0);
						(*iter)->processMouseExited(event);
					}
				}
			}

            if(isExpand() && m_expandMenu)
			{
                m_expandMenu->listMouseMotion(e);
			}
        }

		void MenuBar::updateLayout()
		{
			std::vector<Menu*>::iterator iter;
            int tempBegin=m_rightSpacer;
            for(iter=m_menuList.begin();iter<m_menuList.end();++iter)
			{
                (*iter)->m_position.x=tempBegin;
                (*iter)->m_position.y=m_topSpacer;
                tempBegin+=m_spacer+(*iter)->getPreferedSize().m_width;
			}
        }
	}
}
