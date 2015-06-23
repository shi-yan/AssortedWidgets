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
			unsigned int length;
			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;
		public:
			TextField(unsigned int _length);
			TextField(unsigned int _length,char *_text);
			TextField(unsigned int _length,std::string &_text);
			unsigned int getLength()
			{
				return length;
			};
			unsigned int getTop()
			{
				return top;
			}
			unsigned int getBottom()
			{
				return bottom;
			}
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getTextFieldPreferedSize(this);
			};
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintTextField(this);
			};
		public:
			~TextField(void);
		};
	}
}