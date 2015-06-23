#pragma once
#include "MenuItem.h"
#include "MenuItemRadioGroup.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItemRadioButton:public MenuItem
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
			int style;
			int status;
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
			int getStatus()
			{
				return status;
			};
			void setGroup(MenuItemRadioGroup *_group)
			{
				group=_group;
			};
			MenuItemRadioGroup* getGroup()
			{
				return group;
			};
			void setToggle(bool _toggle)
			{
				toggle=_toggle;
			};
			bool getToggle()
			{
				return toggle;
			};
			void mousePressed(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);

			void mouseReleased(const Event::MouseEvent &e);

			void mouseExited(const Event::MouseEvent &e);

			
			MenuItemRadioButton(std::string &_text);
			MenuItemRadioButton(char *_text);
			std::string getText()
			{
				return text;
			};
			int getStyle()
			{
				return style;
			}
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemRadioButton(this);
			};

			unsigned int getLeft()
			{
				return left;
			};

			unsigned int getRight()
			{
				return right;
			};

			unsigned int getBottom()
			{
				return bottom;
			};

			unsigned int getTop()
			{
				return top;
			};

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemRadioButtonPreferedSize(this);
			};
		public:
			~MenuItemRadioButton(void);
		};
	}
}