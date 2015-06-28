#pragma once

#include "FreeTypeFont.h"

namespace AssortedWidgets
{
	namespace Font
	{
		class FontEngine
		{
		private:
            FreeTypeFont m_freeTypeFont;
            FontEngine(void)
                :m_freeTypeFont("arial.ttf", 10)
            {}
		public:
			static FontEngine &getSingleton()
			{
				static FontEngine obj;
				return obj;
			}
			FreeTypeFont &getFont()
			{
                return m_freeTypeFont;
            }
		private:
			~FontEngine(void);
		};
	}
}
