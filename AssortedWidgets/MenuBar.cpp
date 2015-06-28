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
			if(expandMenu)
			{
				expandMenu->shrink();
			}
			expandMenu=_expandMenu;
			expand=true;
		};

		void MenuBar::setShrink()
		{
			if(expandMenu)
			{
				expandMenu->shrink();
			}
			expandMenu=0;
			expand=false;
		};

		void MenuBar::addMenu(Menu *menu)
		{
			menuList.push_back(menu);
			menu->setMenuBar(this);
			updateLayout();
		};

		void MenuBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintMenuBar(this);
			std::vector<Menu*>::iterator iter;
			for(iter=menuList.begin();iter<menuList.end();++iter)
			{
				(*iter)->paint();
			}
		};

		void MenuBar::onMouseEnter(const Event::MouseEvent &e)
		{
			isHover=true;
			onMouseMove(e);
		};

		void MenuBar::onMouseExit(const Event::MouseEvent &e)
		{
			isHover=false;
			onMouseMove(e);
		};

		void MenuBar::onMousePressed(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
			for(iter=menuList.begin();iter<menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_PRESSED,e.getX(),e.getY(),0);
					(*iter)->processMousePressed(event);
				}
			}

			if(isExpand() && expandMenu)
			{
				expandMenu->listMousePressed(e);
			}
		};

		void MenuBar::onMouseReleased(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
			for(iter=menuList.begin();iter<menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
					Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_RELEASED,e.getX(),e.getY(),0);
					(*iter)->processMouseReleased(event);
				}
			}
			if(isExpand() && expandMenu)
			{
				expandMenu->listMouseReleased(e);
			}
		};

		void MenuBar::onMouseMove(const Event::MouseEvent &e)
		{
			std::vector<Menu*>::iterator iter;
			for(iter=menuList.begin();iter<menuList.end();++iter)
			{
				if((*iter)->isIn(e.getX(),e.getY()))
				{
					if(!(*iter)->isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,e.getX(),e.getY(),0);
						(*iter)->processMouseEntered(event);
					}
				}
				else
				{
					if((*iter)->isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,e.getX(),e.getY(),0);
						(*iter)->processMouseExited(event);
					}
				}
			}

			if(isExpand() && expandMenu)
			{
				expandMenu->listMouseMotion(e);
			}
		};

		void MenuBar::updateLayout()
		{
			std::vector<Menu*>::iterator iter;
			int tempBegin=rightSpacer;
			for(iter=menuList.begin();iter<menuList.end();++iter)
			{
                (*iter)->m_position.x=tempBegin;
                (*iter)->m_position.y=topSpacer;
				tempBegin+=spacer+(*iter)->getPreferedSize().width;
			}
		};
	}
}
