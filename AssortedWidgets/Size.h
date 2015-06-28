#pragma once

namespace AssortedWidgets
{
	namespace Util
	{
		class Size
		{
		public:
            unsigned int m_width;
            unsigned int m_height;
		public:
            Size(void)
                :m_width(0),
                  m_height(0)
            {}
            Size(unsigned int _width,unsigned int _height)
                :m_width(_width),
                  m_height(_height)
            {}
            Size(unsigned int size)
                :m_width(size),
                  m_height(size)
            {}
            Size(const Size& in)
                :m_width(in.m_width),
                  m_height(in.m_height)
            {}
			void operator=(const Size& in)
			{
                m_width=in.m_width;
                m_height=in.m_height;
            }
			void operator+=(const Size& in)
			{
                m_width+=in.m_width;
                m_height+=in.m_height;
            }
			void operator+=(const int offset)
			{
                m_width+=offset;
                m_height+=offset;
            }

			bool operator==(const Size& in)
			{
                return ((m_width==in.m_width)&&(m_height==in.m_height));
            }

			void operator-=(const Size& in)
			{
                m_width-=in.m_width;
                m_height-=in.m_height;
            }

			void operator-=(const int offset)
			{
                m_width-=offset;
                m_height-=offset;
			}
		public:
            ~Size(void){}
		};
			
		inline Size operator+(const Size& a,const Size& b)
		{
            Size result(a.m_width+b.m_width,a.m_height+b.m_height);
			return result;
		}

		inline Size operator-(const Size& a,const Size& b)
		{
            Size result(a.m_width-b.m_width,a.m_height-b.m_height);
			return result;
		}

		inline Size operator+(const Size& o,int offset)
		{
            Size result(o.m_width+offset,o.m_height+offset);
			return result;
		}
	}
}
