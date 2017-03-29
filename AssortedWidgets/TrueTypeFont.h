#pragma once

#include "SDL2/SDL_opengl.h"
#include "Font.h"
#include <vector>
#include "fontstash.h"

namespace AssortedWidgets
{
	namespace Font
	{
        class TrueTypeFont: public Font
		{
		private:
            struct sth_stash* m_stash;
            int m_font;
            int m_size;

		public:
            TrueTypeFont(const char* _fontName,size_t _size);

			Util::Size getStringBoundingBox(const std::string &text) const;

			void drawString(int x, int y, const std::string &text) const;

			void printf(int x,int y,const char *fmt, ...) const;

		public:
            ~TrueTypeFont(void);
		};
	}
}
