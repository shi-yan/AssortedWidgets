#pragma once

#ifdef __APPLE__
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GL/gl.h>
#include <GL/glu.h>
#endif
#include <stack>
#include "Position.h"

namespace AssortedWidgets
{
	namespace Util
	{
		class Graphics
		{
		private:
            std::stack<Position> m_positionStack;
		private:
            Graphics(){}
		public:
			static Graphics& getSingleton()
			{
				static Graphics obj;
				return obj;
            }
            void pushPosition(Position &newPosition)
			{
                if(m_positionStack.empty())
				{
                    m_positionStack.push(newPosition);
				}
				else
				{
                    newPosition.x+=m_positionStack.top().x;
                    newPosition.y+=m_positionStack.top().y;
                    m_positionStack.push(newPosition);
				}
            }
			Position popPosition()
			{
                Position result=m_positionStack.top();
                m_positionStack.pop();
				return result;
            }
			Position getOrigin()
			{
                if(m_positionStack.empty())
				{
					return Util::Position(0,0);
				}
				else
				{
                    return m_positionStack.top();
				}
			}
		private:
            ~Graphics(void){}
		};
	}
}
