#pragma once
#include "MenuItem.h"
#include "MenuList.h"
#include "MouseEvent.h"
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItemSubMenu:public MenuItem
		{
		public:
			enum Status
			{
				hover,
				normal,
				pressed
			};
		private:
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			unsigned int top;
			bool expand;
			std::string text;
			int status;
			MenuList menuList;

		public:
			bool isExpand()
			{
				return expand;
			};
			void mousePressed(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);

			void mouseReleased(const Event::MouseEvent &e);

			void mouseExited(const Event::MouseEvent &e);

			void listMouseMotion(const Event::MouseEvent &e);
			void listMousePressed(const Event::MouseEvent &e);
			void listMouseReleased(const Event::MouseEvent &e);

			void shrink()
			{
				expand=false;
				status=normal;
			};

			std::string getText() const
			{
				return text;
			};

			int getStatus() const
			{
				return status;
			};
			unsigned int getLeft()
			{
				return left;
			};

			unsigned int getRight()
			{
				return right;
			};

			unsigned int getBottom()
			{
				return bottom;
			};

			unsigned int getTop()
			{
				return top;
			};
			void addItem(MenuItem *item)
			{
				menuList.addItem(item);
			};

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemSubMenuPreferedSize(this);
			};

			void paint(void)
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemSubMenu(this);
				if(expand && !menuList.getItemList().empty())
				{
                    Util::Position p(position);
                    Util::Graphics::getSingleton().pushPosition(p);
					menuList.paint();
					Util::Graphics::getSingleton().popPosition();
				}
			};

			MenuItemSubMenu(std::string &_text);
			MenuItemSubMenu(char *_text);
		public:
			~MenuItemSubMenu(void);
		};
	}
}
