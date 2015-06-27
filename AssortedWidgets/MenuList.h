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
            }

			bool isExpand()
			{
				return expand;
            }

			void setExpand(MenuItemSubMenu *_expandSubMenu);

			void setShrink();

			MenuList(void);
			void addItem(MenuItem *item);
			
			std::vector<MenuItem *> &getItemList()
			{
				return itemList;
            }
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
            unsigned int getSpacer() const
			{
				return spacer;
            }
            unsigned int getTop() const
			{
				return top;
            }
            unsigned int getBottom() const
			{
				return bottom;
            }
            unsigned int getLeft() const
			{
				return left;
            }
            unsigned int getRight() const
			{
				return right;
            }
            unsigned int getMinimizeSize() const
			{
				return minimizeSize;
            }
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuListPreferedSize(this);
            }

			void updateLayout();
		public:
			~MenuList(void);
		};
	}
}
