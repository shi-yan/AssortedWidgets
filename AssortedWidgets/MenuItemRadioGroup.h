#pragma once
#include "MenuItem.h"
#include <vector>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItemRadioButton;

		class MenuItemRadioGroup:public MenuItem
		{
		private:
			unsigned int top;
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			unsigned int spacer;
			unsigned int minimizeSize;
			std::vector<MenuItemRadioButton*> itemList;
			MenuItemRadioButton *currentSelection;
		public:
			unsigned int getSpacer()
			{
				return spacer;
			};
			std::vector<MenuItemRadioButton *> &getItemList()
			{
				return itemList;
			};
			unsigned int getTop()
			{
				return top;
			};
			unsigned int getBottom()
			{
				return bottom;
			};
			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
	
			unsigned int getLeft()
			{
				return left;
			};

			void setSelection(size_t index);
			int getSelection();
			void setSelection(MenuItemRadioButton *item);

			unsigned int getRight()
			{
				return right;
			};
			unsigned int getMinimizeSize()
			{
				return minimizeSize;
			};
			void addItem(MenuItemRadioButton *item);
			void updateLayout();
			Util::Size getPreferedSize(void)
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemRadioGroupPreferedSize(this);
			};
			void paint();
			MenuItemRadioGroup(void);
		public:
			~MenuItemRadioGroup(void);
		};
	}
}