#pragma once
#include "MenuItem.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        class MenuItemSeparator: public MenuItem
		{
		public:
			MenuItemSeparator(void);
			Util::Size getPreferedSize(void)
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getMenuItemSeparatorPreferedSize(this);
            }
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintMenuItemSeparator(this);
            }
		public:
			~MenuItemSeparator(void);
		};
	}
}
