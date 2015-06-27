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
            unsigned int m_top;
            unsigned int m_left;
            unsigned int m_right;
            unsigned int m_bottom;
            unsigned int m_spacer;
            unsigned int m_minimizeSize;
            std::vector<MenuItemRadioButton*> m_itemList;
            MenuItemRadioButton *m_currentSelection;
		public:
            unsigned int getSpacer() const
			{
                return m_spacer;
            }
			std::vector<MenuItemRadioButton *> &getItemList()
			{
                return m_itemList;
            }
            unsigned int getTop() const
			{
                return m_top;
            }
            unsigned int getBottom() const
			{
                return m_bottom;
            }
			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
	
            unsigned int getLeft() const
			{
                return m_left;
            }

			void setSelection(size_t index);
			int getSelection();
			void setSelection(MenuItemRadioButton *item);

			unsigned int getRight()
			{
                return m_right;
            }
			unsigned int getMinimizeSize()
			{
                return m_minimizeSize;
            }
			void addItem(MenuItemRadioButton *item);
			void updateLayout();
			Util::Size getPreferedSize(void)
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemRadioGroupPreferedSize(this);
            }
			void paint();
			MenuItemRadioGroup(void);
		public:
			~MenuItemRadioGroup(void);
		};
	}
}
