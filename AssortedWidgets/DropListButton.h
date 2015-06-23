#pragma once
#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DropListButton:public AbstractButton
		{
		public:
			Util::Size getPreferedSize()
			{
				return Util::Size(15,15);
			};

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDropListButton(this);
			};
			DropListButton(void);
		public:
			~DropListButton(void);
		};
	}
}