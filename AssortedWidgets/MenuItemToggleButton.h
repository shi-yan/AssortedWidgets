#pragma once
#include "MenuItem.h"
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItemToggleButton:public MenuItem
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
            bool m_toggle;

		public:
            enum Status getStatus() const
			{
                return m_status;
            }

			void setToggle(bool _toggle)
			{
                m_toggle=_toggle;
            }

            bool getToggle() const
			{
                return m_toggle;
            }

			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);

			MenuItemToggleButton(std::string &_text);
			MenuItemToggleButton(char *_text);

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
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemToggleButton(this);
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
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemToggleButtonPreferedSize(this);
            }

		public:
			~MenuItemToggleButton(void);
		};
	}
}
