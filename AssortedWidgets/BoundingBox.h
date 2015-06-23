#pragma once

#include "Position.h"
#include "Size.h"

namespace AssortedWidgets
{
	namespace Util
	{
		class BoundingBox
		{
		public:
			Position position;
			Size size;
		public:
			BoundingBox(void):position(0,0),size(0,0)
			{};
			BoundingBox(int x,int y,unsigned int width,unsigned int height):position(x,y),size(width,height)
			{};
			BoundingBox(const BoundingBox& in):position(in.position),size(in.size)
			{};
			void operator=(const BoundingBox& in)
			{
				position=in.position;
				size=in.size;
			};
			bool operator==(const BoundingBox& in)
			{
				return ((position==(in.position))&&(size==in.size));
			};
			bool isIn(int x,int y)
			{
				return ((position.x<x)&&(x<(position.x+static_cast<int>(size.width)))&&(position.y<y)&&(y<(position.y+static_cast<int>(size.height))));
			};
		public:
			~BoundingBox(void){};
		};
	}
}
