#pragma once
#include "Component.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Logo:public Component
		{
		public:
			Logo(void)
;
			Util::Size getPreferedSize()
			{
				return Util::Size(253,87);
			};
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintLogo(this);
			};
		public:
			~Logo(void);
		};
	}
}