#pragma once
#include "AbstractButton.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ScrollBarButton:public AbstractButton
		{
		public:
			enum Type
			{
				HorizontalLeft,
				HorizontalRight,
				VerticalTop,
				VerticalBottom
			};
		private:
            int m_type;
		public:
            int getType() const
			{
                return m_type;
            }
			ScrollBarButton(int _type);

			Util::Size getPreferedSize()
			{
				return Util::Size(15,15);
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintScrollBarButton(this);
			};

		public:
			~ScrollBarButton(void);
		};
	}
}
