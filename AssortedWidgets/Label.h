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
            }

            bool isDrawBackground() const
			{
				return drawBackground;
            }

            const std::string &getText() const
			{
				return text;
            }

            unsigned int getTop() const
			{
				return top;
            }

			void setText(char *_text)
			{
				text=_text;
			}

            void setText(const std::string &_text)
			{
				text=_text;
			}

            unsigned int getLeft() const
			{
				return left;
            }

            unsigned int getRight() const
			{
				return right;
            }

            unsigned int getBottom() const
			{
				return bottom;
            }

            Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getLabelPreferedSize(this);
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintLabel(this);
            }

			Label(std::string &_text);
			Label(char *_text);
		public:
			~Label(void);
		};
	}
}
