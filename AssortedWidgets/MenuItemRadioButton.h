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
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			unsigned int top;
			std::string text;
            enum Style style;
            enum Status status;
			bool toggle;
			MenuItemRadioGroup *group;

		public:
			void off()
			{
				toggle=false;
			}

			void on()
			{
				toggle=true;
            }

            enum Status getStatus() const
			{
				return status;
            }

			void setGroup(MenuItemRadioGroup *_group)
			{
				group=_group;
            }

			MenuItemRadioGroup* getGroup()
			{
				return group;
            }

			void setToggle(bool _toggle)
			{
				toggle=_toggle;
            }

            bool getToggle() const
			{
				return toggle;
            }

			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);

            MenuItemRadioButton(std::string &_text);
			MenuItemRadioButton(char *_text);
            const std::string& getText() const
			{
				return text;
            }
			int getStyle()
			{
				return style;
			}
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemRadioButton(this);
            }

            unsigned int getLeft() const
			{
				return left;
            }

            unsigned int getRight() const
			{
				return right;
            }

            unsigned int getBottom() const
			{
				return bottom;
            }

            unsigned int getTop() const
			{
				return top;
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
