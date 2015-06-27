#pragma once
#include "ContainerElement.h"
#include <string>
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Label:public Element
		{
		private:
			std::string text;
			unsigned int top;
			unsigned int left;
			unsigned int right;
			unsigned int bottom;
			bool drawBackground;
		public:
			void setDrawBackground(bool _drawBackground)
			{
				drawBackground=_drawBackground;
			};
			bool isDrawBackground()
			{
				return drawBackground;
			};
			std::string getText()
			{
				return text;
			};

			unsigned int getTop()
			{
				return top;
			};

			void setText(char *_text)
			{
				text=_text;
			}

            void setText(const std::string &_text)
			{
				text=_text;
			}

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

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getLabelPreferedSize(this);
			};

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintLabel(this);
			};

			Label(std::string &_text);
			Label(char *_text);
		public:
			~Label(void);
		};
	}
}
