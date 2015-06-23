#pragma once

namespace AssortedWidgets
{
	namespace Util
	{
		class Size
		{
		public:
			unsigned int width;
			unsigned int height;
		public:
			Size(void):width(0),height(0)
			{};
			Size(unsigned int _width,unsigned int _height):width(_width),height(_height)
			{};
			Size(unsigned int size):width(size),height(size)
			{};
			Size(const Size& in):width(in.width),height(in.height)
			{};
			void operator=(const Size& in)
			{
				width=in.width;
				height=in.height;
			};
			void operator+=(const Size& in)
			{
				width+=in.width;
				height+=in.height;
			};
			void operator+=(const int offset)
			{
				width+=offset;
				height+=offset;
			};

			bool operator==(const Size& in)
			{
				return ((width==in.width)&&(height==in.height));
			};

			//如果得负的话会有隐患
			void operator-=(const Size& in)
			{
				width-=in.width;
				height-=in.height;
			};
			//如果得负的话会有隐患
			void operator-=(const int offset)
			{
				width-=offset;
				height-=offset;
			}
		public:
			~Size(void){};
		};
			
		inline Size operator+(const Size& a,const Size& b)
		{
			Size result(a.width+b.width,a.height+b.height);
			return result;
		}

		inline Size operator-(const Size& a,const Size& b)
		{
			Size result(a.width-b.width,a.height-b.height);
			return result;
		}

		inline Size operator+(const Size& o,int offset)
		{
			Size result(o.width+offset,o.height+offset);
			return result;
		}
	}
}