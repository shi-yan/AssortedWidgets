#pragma once
#include "ContainerElement.h"
#include <string>
#include "ThemeEngine.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class AbstractButton:public Element
		{
		public:
			enum Status
			{
				normal,
				hover,
				pressed
			};
		private:
			unsigned int top;
			unsigned int bottom;
			unsigned int right;
			unsigned int left;
			int status;
		public:
			AbstractButton(void);
			AbstractButton(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right);
			AbstractButton(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right,int _status);
			
			unsigned int getTop()
			{
				return top;
			};

			unsigned int getBottom()
			{
				return bottom;
			};

			unsigned int getRight()
			{
				return right;
			};

			unsigned int getLeft()
			{
				return left;
			};

			int getStatus()
			{
				return status;
			};

			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);

		public:
			~AbstractButton(void);
		};
	}
}