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
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			unsigned int top;
			std::string text;
			int style;
			int status;
		public:
			int getStatus()
			{
				return status;
			};

			void mousePressed(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);

			void mouseReleased(const Event::MouseEvent &e);

			void mouseExited(const Event::MouseEvent &e);

			
			MenuItemButton(std::string &_text);
			MenuItemButton(char *_text);
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
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemButton(this);
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
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemButtonPreferedSize(this);
			};
		public:
			~MenuItemButton(void);
		};
	}
}