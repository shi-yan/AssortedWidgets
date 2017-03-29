#pragma once
#include <string>
#include "Component.h"
#include "ThemeEngine.h"
#include "MenuList.h"
#include "Graphics.h"
#include "MenuItem.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuBar;

        class Menu: public Component
		{
		public:
			enum Status
			{
				normal,
				hover,
				pressed
			};
		private:
            bool m_expand;
            std::string m_text;
            enum Status m_status;
            MenuList m_menuList;
            MenuBar *m_menuBar;

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

			void setMenuBar(MenuBar *_menuBar)
			{
                m_menuBar=_menuBar;
            }

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

			void addItem(MenuItem *item)
			{
                m_menuList.addItem(item);
            }

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuPreferedSize(this);
            }

			void paint(void)
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenu(this);
                if(m_expand && !m_menuList.getItemList().empty())
				{
                    Util::Position p(m_position.x,m_position.y);
                    Util::Graphics::getSingleton().pushPosition(p);
                    m_menuList.paint();
					Util::Graphics::getSingleton().popPosition();
				}
            }

            Menu(const std::string &_text);
            Menu(const char *_text);
		public:
			~Menu(void);
		};
	}
}
