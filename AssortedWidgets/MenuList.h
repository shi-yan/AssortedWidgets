#pragma once
#include "Component.h"
#include <vector>
#include "ThemeEngine.h"
#include "Graphics.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItem;
		class MenuItemSubMenu;

		class MenuList:public Component
		{
		private:
			std::vector<MenuItem*> itemList;
			unsigned int minimizeSize;
			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;
			unsigned int spacer;

		private:
			MenuItemSubMenu *expandSubMenu;
			bool expand;

		public:

			MenuItemSubMenu* getExpandMenu()
			{
				return expandSubMenu;
			};

			bool isExpand()
			{
				return expand;
			};

			void setExpand(MenuItemSubMenu *_expandSubMenu);

			void setShrink();

			MenuList(void);
			void addItem(MenuItem *item);
			
			std::vector<MenuItem *> &getItemList()
			{
				return itemList;
			};
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
			unsigned int getSpacer()
			{
				return spacer;
			};
			unsigned int getTop()
			{
				return top;
			};
			unsigned int getBottom()
			{
				return bottom;
			};
			unsigned int getLeft()
			{
				return left;
			};
			unsigned int getRight()
			{
				return right;
			};
			unsigned int getMinimizeSize()
			{
				return minimizeSize;
			};
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuListPreferedSize(this);
			};

			void updateLayout();
		public:
			~MenuList(void);
		};
	}
}