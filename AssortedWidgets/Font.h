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
			std::string fontName;
			size_t size;
		public:
			Font(char* _fontName,size_t _size):fontName(_fontName),size(_size)
			{};
			std::string getFontName() const
			{
				return fontName;
			};
			size_t getSize() const
			{
				return size;
			};
			virtual Util::Size getStringBoundingBox(const std::string &text) const = 0;
			virtual void drawString(int x, int y, const std::string &text) const = 0;
			virtual void printf(int x,int y,const char *fmt, ...) const =0;
			virtual ~Font();
		};
	}
}