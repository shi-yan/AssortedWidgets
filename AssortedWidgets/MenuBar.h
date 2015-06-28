#pragma once
#include <vector>
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Menu;
		class MenuBar:public Component
		{
		private:
			std::vector<Menu*> menuList;
			std::vector<Component*> inList;
			int spacer;
			int leftSpacer;
			int topSpacer;
			int rightSpacer;
			int bottomSpacer;

			Menu *expandMenu;
			bool expand;

		private:
			MenuBar():leftSpacer(45),topSpacer(5),rightSpacer(45),bottomSpacer(5),expand(false),expandMenu(0)
            {
                mouseMovedHandlerList.push_back(MOUSE_DELEGATE(MenuBar::onMouseMove));
                mouseEnteredHandlerList.push_back(MOUSE_DELEGATE(MenuBar::onMouseEnter));
                mouseExitedHandlerList.push_back(MOUSE_DELEGATE(MenuBar::onMouseExit));
                mousePressedHandlerList.push_back(MOUSE_DELEGATE(MenuBar::onMousePressed));
                mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MenuBar::onMouseReleased));
            }

		public:
			void init(unsigned int width,unsigned int height=30,int _spacer=5)
			{
				spacer=_spacer;
                m_position.x=0;
                m_position.y=0;
                m_size.width=width;
                m_size.height=30;
            }

			static MenuBar& getSingleton()
			{
				static MenuBar obj;
				return obj;
			}

			Menu* getExpandMenu()
			{
				return expandMenu;
			};

			bool isExpand()
			{
				return expand;
			};

			void setExpand(Menu *_expandMenu);

			void setShrink();

			void addMenu(Menu *menu);

			void paint();

			Util::Size getPreferedSize()
			{
                return m_size;
            }

			void onMouseEnter(const Event::MouseEvent &e);

			void onMouseExit(const Event::MouseEvent &e);

			void onMousePressed(const Event::MouseEvent &e);

			void onMouseReleased(const Event::MouseEvent &e);

			void onMouseMove(const Event::MouseEvent &e);

			void updateLayout();


		public:
			~MenuBar(void);
		};
	}
}
