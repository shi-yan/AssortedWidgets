#pragma once
#include "MenuItem.h"
#include "MenuItemRadioGroup.h"
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
        class MenuItemRadioButton: public MenuItem
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
            MenuItemRadioGroup *m_group;

		public:
			void off()
			{
                m_toggle=false;
			}

			void on()
			{
                m_toggle=true;
            }

            enum Status getStatus() const
			{
                return m_status;
            }

			void setGroup(MenuItemRadioGroup *_group)
			{
                m_group=_group;
            }

			MenuItemRadioGroup* getGroup()
			{
                return m_group;
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

            MenuItemRadioButton(const std::string &_text);
            MenuItemRadioButton(const char *_text);
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
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemRadioButton(this);
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
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemRadioButtonPreferedSize(this);
            }
		public:
			~MenuItemRadioButton(void);
		};
	}
}
