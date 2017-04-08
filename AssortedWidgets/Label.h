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
            std::string m_text;
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            bool m_drawBackground;

		public:
			void setDrawBackground(bool _drawBackground)
			{
                m_drawBackground=_drawBackground;
            }

            bool isDrawBackground() const
			{
                return m_drawBackground;
            }

            const std::string &getText() const
			{
                return m_text;
            }

            unsigned int getTop() const
			{
                return m_top;
            }

			void setText(char *_text)
			{
                m_text=_text;
			}

            void setText(const std::string &_text)
			{
                m_text=_text;
			}

            unsigned int getLeft() const
			{
                return m_left;
            }

            unsigned int getRight() const
			{
                return m_right;
            }

            unsigned int getBottom() const
			{
                return m_bottom;
            }

            Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getLabelPreferedSize(this);
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintLabel(this);
            }

            Label(const std::string &_text);
		public:
			~Label(void);
		};
	}
}
