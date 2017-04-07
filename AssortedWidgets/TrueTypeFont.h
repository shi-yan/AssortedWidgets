#pragma once

#include "Font.h"
#include <vector>
#include <map>

struct FONScontext;

namespace AssortedWidgets
{
	namespace Font
	{
        class TrueTypeFont: public Font
		{
		private:
            struct FONScontext* m_stash;
            int m_font;
            int m_size;
            unsigned int  m_textBuffer;
            int m_fontNormal;
            unsigned int m_color;

            std::map<std::string, int> m_textIDs;

		public:
            TrueTypeFont(const char* _fontName,size_t _size);

            Util::Size getStringBoundingBox(const std::string &text) ;

            void drawString(int x, int y, const std::string &text) ;

            void printf(int x,int y,const char *fmt, ...) ;

            int findTextID(const std::string &text, bool &isNew);

            void setColor(int r, int g, int b);

		public:
            ~TrueTypeFont(void);
		};
	}
}
