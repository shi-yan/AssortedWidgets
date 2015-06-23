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
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			unsigned int top;
			std::string text;
			int style;
			int status;
			bool toggle;
		public:
			int getStatus()
			{
				return status;
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

			
			MenuItemToggleButton(std::string &_text);
			MenuItemToggleButton(char *_text);
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
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemToggleButton(this);
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
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemToggleButtonPreferedSize(this);
			};
		public:
			~MenuItemToggleButton(void);
		};
	}
}