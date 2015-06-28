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
            std::vector<Menu*> m_menuList;
            std::vector<Component*> m_inList;
            int m_spacer;
            int m_leftSpacer;
            int m_topSpacer;
            int m_rightSpacer;
            int m_bottomSpacer;

            Menu *m_expandMenu;
            bool m_expand;

		private:
            MenuBar()
                :m_leftSpacer(45),
                  m_topSpacer(5),
                  m_rightSpacer(45),
                  m_bottomSpacer(5),
                  m_expand(false),
                  m_expandMenu(0)
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
                m_spacer=_spacer;
                m_position.x=0;
                m_position.y=0;
                m_size.m_width=width;
                m_size.m_height=30;
            }

			static MenuBar& getSingleton()
			{
				static MenuBar obj;
				return obj;
			}

			Menu* getExpandMenu()
			{
                return m_expandMenu;
            }

            bool isExpand() const
			{
                return m_expand;
            }

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
