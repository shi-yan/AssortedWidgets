#pragma once
#include <string>
#include "BoundingBox.h"

namespace AssortedWidgets
{
	namespace Font
	{
		class Font
		{
		private:
            std::string m_fontName;
            size_t m_size;
		public:
            Font(char* _fontName,size_t _size)
                :m_fontName(_fontName),
                  m_size(_size)
            {}
            const std::string &getFontName() const
			{
                return m_fontName;
            }
			size_t getSize() const
			{
                return m_size;
            }
			virtual Util::Size getStringBoundingBox(const std::string &text) const = 0;
			virtual void drawString(int x, int y, const std::string &text) const = 0;
			virtual void printf(int x,int y,const char *fmt, ...) const =0;
			virtual ~Font();
		};
	}
}
