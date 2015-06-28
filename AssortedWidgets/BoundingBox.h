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
            Position m_position;
            Size m_size;
        public:
            BoundingBox(int x=0,int y=0,unsigned int width=0,unsigned int height=0)
                :m_position(x,y),
                  m_size(width,height)
            {}

            BoundingBox(const BoundingBox& in)
                :m_position(in.m_position),
                  m_size(in.m_size)
            {}

			void operator=(const BoundingBox& in)
			{
                m_position=in.m_position;
                m_size=in.m_size;
            }

			bool operator==(const BoundingBox& in)
			{
                return ((m_position==(in.m_position))&&(m_size==in.m_size));
            }

			bool isIn(int x,int y)
			{
                return ((m_position.x<x)&&(x<(m_position.x+static_cast<int>(m_size.m_width)))&&(m_position.y<y)&&(y<(m_position.y+static_cast<int>(m_size.m_height))));
            }

		public:
            virtual ~BoundingBox(void){}
		};
	}
}
