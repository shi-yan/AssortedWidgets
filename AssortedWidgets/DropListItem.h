#pragma once
#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DropListItem:public AbstractButton
		{
		private:
            std::string m_text;
		public:
            void setString(const std::string &_text)
			{
                m_text=_text;
            }

            const std::string &getText() const
			{
                return m_text;
            }
            DropListItem(const std::string &_text);
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDropListItem(this);
            }
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getDropListItemPreferedSize(this);
            }
		public:
			~DropListItem(void);
		};
	}
}
