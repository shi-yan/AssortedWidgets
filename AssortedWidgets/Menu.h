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
			bool expand;
			std::string text;
            enum Status status;
			MenuList menuList;
			MenuBar *menuBar;

		public:
			bool isExpand()
			{
				return expand;
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
				menuBar=_menuBar;
            }

			void shrink()
			{
				expand=false;
				status=normal;
			}

            const std::string& getText() const
			{
				return text;
            }

            enum Status getStatus() const
			{
				return status;
            }

			void addItem(MenuItem *item)
			{
				menuList.addItem(item);
            }

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuPreferedSize(this);
            }

			void paint(void)
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenu(this);
				if(expand && !menuList.getItemList().empty())
				{
                    Util::Position p(m_position.x,m_position.y);
                    Util::Graphics::getSingleton().pushPosition(p);
					menuList.paint();
					Util::Graphics::getSingleton().popPosition();
				}
            }

			Menu(std::string &_text);
			Menu(char *_text);
		public:
			~Menu(void);
		};
	}
}
