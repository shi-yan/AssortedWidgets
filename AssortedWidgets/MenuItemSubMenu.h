#pragma once
#include "MenuItem.h"
#include "MenuList.h"
#include "MouseEvent.h"
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
        class MenuItemSubMenu: public MenuItem
		{
		public:
			enum Status
			{
				hover,
				normal,
				pressed
			};
		private:
            unsigned int m_left;
            unsigned int m_right;
            unsigned int m_bottom;
            unsigned int m_top;
            bool m_expand;
            std::string m_text;
            enum Status m_status;
            MenuList m_menuList;

		public:
            bool isExpand() const
			{
                return m_expand;
            }
			void mousePressed(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);

			void mouseReleased(const Event::MouseEvent &e);

			void mouseExited(const Event::MouseEvent &e);

			void listMouseMotion(const Event::MouseEvent &e);
			void listMousePressed(const Event::MouseEvent &e);
			void listMouseReleased(const Event::MouseEvent &e);

			void shrink()
			{
                m_expand=false;
                m_status=normal;
            }

            const std::string& getText() const
			{
                return m_text;
            }

            enum Status getStatus() const
			{
                return m_status;
            }

            unsigned int getLeft() const
			{
                return m_left;
            }

            unsigned int getRight() const
			{
                return m_right;
            }

            unsigned int getBottom() const
			{
                return m_bottom;
            }

            unsigned int getTop() const
			{
                return m_top;
            }

			void addItem(MenuItem *item)
			{
                m_menuList.addItem(item);
            }

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemSubMenuPreferedSize(this);
            }

			void paint(void)
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemSubMenu(this);
                if(m_expand && !m_menuList.getItemList().empty())
				{
                    Util::Position p(m_position);
                    Util::Graphics::getSingleton().pushPosition(p);
                    m_menuList.paint();
					Util::Graphics::getSingleton().popPosition();
				}
            }

            MenuItemSubMenu(const std::string &_text);
            MenuItemSubMenu(const char *_text);
		public:
			~MenuItemSubMenu(void);
		};
	}
}
