#pragma once
#include "MenuItem.h"
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItemButton:public MenuItem
		{
		public:
			enum Style
			{
				any,
				shrink,
				stretch
			};

			enum Status
			{
				normal,
				hover,
				pressed
			};
		private:
            unsigned int m_left;
            unsigned int m_right;
            unsigned int m_bottom;
            unsigned int m_top;
            std::string m_text;
            enum Style m_style;
            enum Status m_status;
		public:
            enum Status getStatus() const
			{
                return m_status;
            }

			void mousePressed(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);

			void mouseReleased(const Event::MouseEvent &e);

			void mouseExited(const Event::MouseEvent &e);
			
            MenuItemButton(const std::string &_text);
            const std::string& getText() const
			{
                return m_text;
            }

            enum Style getStyle() const
			{
                return m_style;
			}

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemButton(this);
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

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemButtonPreferedSize(this);
            }
		public:
			~MenuItemButton(void);
		};
	}
}
