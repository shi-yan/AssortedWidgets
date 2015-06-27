#pragma once
#include "TypeAble.h"
#include <string>
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class TextField:public TypeAble
		{
		private:
            unsigned int m_length;
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
		public:
			TextField(unsigned int _length);
			TextField(unsigned int _length,char *_text);
			TextField(unsigned int _length,std::string &_text);
			unsigned int getLength()
			{
                return m_length;
            }
			unsigned int getTop()
			{
                return m_top;
			}
			unsigned int getBottom()
			{
                return m_bottom;
			}
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getTextFieldPreferedSize(this);
            }
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintTextField(this);
			};
		public:
			~TextField(void);
		};
	}
}
