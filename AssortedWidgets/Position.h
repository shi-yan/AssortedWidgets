#pragma once

namespace AssortedWidgets
{
	namespace Util
	{
		class Position
		{
		public:
			int x;
			int y;

		public:
            Position(int _x = 0,int _y = 0):x(_x),y(_y)
            {}

			Position(const Position &in):x(in.x),y(in.y)
            {}

			void operator=(const Position &in)
			{
				x=in.x;
				y=in.y;
            }

			void operator+=(const Position &in)
			{
				x+=in.x;
				y+=in.y;
            }

			void operator-=(const Position &in)
			{
				x-=in.x;
				y-=in.y;
            }

			bool operator==(const Position &in)
			{
				return ((x==in.x)&&(y=in.y));
            }
		public:
            ~Position(void){}
		};
	}
}
