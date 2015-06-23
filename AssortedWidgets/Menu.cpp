#include "Menu.h"
#include "MenuBar.h"
#include "FontEngine.h"
#include "MenuItemSubMenu.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		Menu::Menu(std::string &_text):text(_text),status(normal),expand(false),menuBar(0)
		{
			size=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(text);
			size.width+=12;
			size.height=20;
			position.x=100;
			position.y=100;

			MouseDelegate mEntered;
			mEntered.bind(this,&Menu::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&Menu::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&Menu::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&Menu::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			menuList.position.x=-9;
			menuList.position.y=25;
		}

		Menu::Menu(char *_text):text(_text),status(normal),expand(false),menuBar(0)
		{
			size=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(text);
			size.width+=12;
			size.height=20;
			position.x=100;
			position.y=100;
			MouseDelegate mEntered;
			mEntered.bind(this,&Menu::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);

			MouseDelegate mExited;
			mExited.bind(this,&Menu::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&Menu::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&Menu::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			menuList.position.x=-9;
			menuList.position.y=25;
		}

		void Menu::mouseReleased(const Event::MouseEvent &e)
		{
			status=hover;
			if(expand)
			{
				menuBar->setShrink();
				expand=false;
			}
			else
			{
				menuBar->setExpand(this);
				expand=true;
			}
		}

		void Menu::listMousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(expand && menuList.isIn(mx,my))
			{
				Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
				menuList.processMousePressed(event);
			}

			if(menuList.isExpand() && menuList.getExpandMenu())
			{
				Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_PRESSED,mx-menuList.position.x,my-menuList.position.y,0);
				menuList.getExpandMenu()->listMousePressed(event);
			}
		}

		void Menu::listMouseReleased(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(expand && menuList.isIn(mx,my))
			{
				Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				menuList.processMouseReleased(event);
			}

			if(menuList.isExpand() && menuList.getExpandMenu())
			{
				Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_RELEASED,mx-menuList.position.x,my-menuList.position.y,0);
				menuList.getExpandMenu()->listMouseReleased(event);
			}
		}

		void Menu::listMouseMotion(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(expand && menuList.isIn(mx,my))
			{
				if(menuList.isHover)
				{
					Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
					menuList.processMouseMoved(event);
				}
				else
				{
					Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					menuList.processMouseEntered(event);
				}
			}
			else
			{
				if(menuList.isHover)
				{
					Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					menuList.processMouseExited(event);
				}
			}

			if(menuList.isExpand() && menuList.getExpandMenu())
			{
				Event::MouseEvent event(&menuList,Event::MouseEvent::MOUSE_MOTION,mx-menuList.position.x,my-menuList.position.y,0);
				menuList.getExpandMenu()->listMouseMotion(event);
			}
		}

		void Menu::mousePressed(const Event::MouseEvent &e)
		{
			status=pressed;
		}

		void Menu::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			if(menuBar->isExpand())
			{
				menuBar->setExpand(this);
				expand=true;
			}
			else
			{
				if(!expand)
				{
					status=hover;
				}
			}
		};

		void Menu::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			if(!expand)
			{
				status=normal;
			}
		};

		Menu::~Menu(void)
		{
		}
	}
}