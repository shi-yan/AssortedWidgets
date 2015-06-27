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
            std::vector<MenuItem*> m_itemList;
            unsigned int m_minimizeSize;
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            unsigned int m_spacer;

		private:
            MenuItemSubMenu *m_expandSubMenu;
            bool m_expand;

		public:

			MenuItemSubMenu* getExpandMenu()
			{
                return m_expandSubMenu;
            }

            bool isExpand() const
			{
                return m_expand;
            }

			void setExpand(MenuItemSubMenu *_expandSubMenu);

			void setShrink();

			MenuList(void);
			void addItem(MenuItem *item);
			
			std::vector<MenuItem *> &getItemList()
			{
                return m_itemList;
            }
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
            unsigned int getSpacer() const
			{
                return m_spacer;
            }
            unsigned int getTop() const
			{
                return m_top;
            }
            unsigned int getBottom() const
			{
                return m_bottom;
            }
            unsigned int getLeft() const
			{
                return m_left;
            }
            unsigned int getRight() const
			{
                return m_right;
            }
            unsigned int getMinimizeSize() const
			{
                return m_minimizeSize;
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
