#pragma once

#include "TrueTypeFont.h"

namespace AssortedWidgets
{
	namespace Font
	{
		class FontEngine
		{
		private:
            TrueTypeFont m_trueTypeFont;
            FontEngine(void)
                :m_trueTypeFont("assets/arial.ttf", 14)
            {}
		public:
			static FontEngine &getSingleton()
			{
				static FontEngine obj;
				return obj;
			}
            TrueTypeFont &getFont()
			{
                return m_trueTypeFont;
            }
		private:
			~FontEngine(void);
		};
	}
}
