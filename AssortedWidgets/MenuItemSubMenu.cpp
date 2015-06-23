#include "MenuItemSubMenu.h"
#include "FontEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{

		MenuItemSubMenu::MenuItemSubMenu(std::string &_text):text(_text),status(normal),expand(false),left(24),top(4),bottom(4),right(2)
		{
			size=getPreferedSize();
			MouseDelegate mEntered;
			mEntered.bind(this,&MenuItemSubMenu::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemSubMenu::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemSubMenu::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemSubMenu::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			menuList.position.x=232-9;
			menuList.position.y=0;
		}

		MenuItemSubMenu::MenuItemSubMenu(char *_text):text(_text),status(normal),expand(false),left(24),top(4),bottom(4),right(2)
		{
			size=getPreferedSize();
			MouseDelegate mEntered;
			mEntered.bind(this,&MenuItemSubMenu::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&MenuItemSubMenu::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&MenuItemSubMenu::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&MenuItemSubMenu::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			menuList.position.x=232-9;
			menuList.position.y=0;
		}

		MenuItemSubMenu::~MenuItemSubMenu(void)
		{
		}

		void MenuItemSubMenu::mouseReleased(const Event::MouseEvent &e)
		{
			if(expand)
			{
				parentMenuList->setShrink();
				expand=false;
			}
			else
			{
				parentMenuList->setExpand(this);
				expand=true;
			}
			status=hover;
		}

		void MenuItemSubMenu::listMousePressed(const Event::MouseEvent &e)
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

		void MenuItemSubMenu::listMouseReleased(const Event::MouseEvent &e)
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

		void MenuItemSubMenu::listMouseMotion(const Event::MouseEvent &e)
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

		void MenuItemSubMenu::mousePressed(const Event::MouseEvent &e)
		{
			status=pressed;
		}

		void MenuItemSubMenu::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			status=hover;//
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

		void MenuItemSubMenu::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			status=normal;
		}
	}
}