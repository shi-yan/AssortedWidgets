#pragma once
#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DropListItem:public AbstractButton
		{
		private:
			std::string text;
		public:
			void setString(std::string &_text)
			{
				text=_text;
			};

            const std::string &getText() const
			{
				return text;
            }
			DropListItem(char *_text);
			DropListItem(std::string &_text);
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDropListItem(this);
			};
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getDropListItemPreferedSize(this);
			};
		public:
			~DropListItem(void);
		};
	}
}
