#pragma once

#include <GL/gl.h>
#include <GL/glu.h>
#include <stack>
#include "Position.h"

namespace AssortedWidgets
{
	namespace Util
	{
		class Graphics
		{
		private:
			std::stack<Position> positionStack;
		private:
			Graphics(){};
		public:
			static Graphics& getSingleton()
			{
				static Graphics obj;
				return obj;
			};
            void pushPosition(Position &newPosition)
			{
				if(positionStack.empty())
				{
					positionStack.push(newPosition);
				}
				else
				{
					newPosition.x+=positionStack.top().x;
					newPosition.y+=positionStack.top().y;
					positionStack.push(newPosition);
				}
			};
			Position popPosition()
			{
				Position result=positionStack.top();
				positionStack.pop();
				return result;
			};
			Position getOrigin()
			{
				if(positionStack.empty())
				{
					return Util::Position(0,0);
				}
				else
				{
					return positionStack.top();
				}
			}
		private:
			~Graphics(void){};
		};
	}
}
