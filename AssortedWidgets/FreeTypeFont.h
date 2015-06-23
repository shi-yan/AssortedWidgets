#pragma once

#include "Font.h"
#include "OGLFT.h"

namespace AssortedWidgets
{
	namespace Font
	{
		class FreeTypeFont: public Font
		{
		private:
			OGLFT::TranslucentTexture *ftfont;
			std::vector<Util::Size> fontCache;
			unsigned int width[256];
			unsigned int height[256];

		public:
			FreeTypeFont(char* _fontName,size_t _size);

			Util::Size getStringBoundingBox(const std::string &text) const;

			void drawString(int x, int y, const std::string &text) const;

			void printf(int x,int y,const char *fmt, ...) const;

		public:
			~FreeTypeFont(void);
		};
	}
}