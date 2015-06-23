#pragma once

#include "FreeTypeFont.h"

namespace AssortedWidgets
{
	namespace Font
	{
		class FontEngine
		{
		private:
			FreeTypeFont freeTypeFont;
			FontEngine(void):freeTypeFont("arial.ttf",10)
			{};
		public:
			static FontEngine &getSingleton()
			{
				static FontEngine obj;
				return obj;
			}
			FreeTypeFont &getFont()
			{
				return freeTypeFont;
			};
		private:
			~FontEngine(void);
		};
	}
}