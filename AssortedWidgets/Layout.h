#pragma once

#include <vector>
#include "Size.h"
#include "Position.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Element;
	}

	namespace Layout
	{
		class Layout
		{
		protected:
			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;
			unsigned int spacer;
		public:
			Layout(void):top(0),bottom(0),left(0),right(0),spacer(2)
			{};
			Layout(unsigned int _spacer):top(0),bottom(0),left(0),right(0),spacer(_spacer)
			{};
			Layout(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right,unsigned int _spacer):top(_top),bottom(_bottom),left(_left),right(_right),spacer(_spacer)
			{};

			void setTop(unsigned int _top)
			{
				top=_top;
			};

			void setBottom(unsigned int _bottom)
			{
				bottom=_bottom;
			};

			void setLeft(unsigned int _left)
			{
				left=_left;
			};

			void setRight(unsigned int _right)
			{
				right=_right;
			};

			void setSpacer(unsigned int _spacer)
			{
				spacer=_spacer;
			};

			unsigned int getTop()
			{
				return top;
			};

			unsigned int getBottom()
			{
				return bottom;
			};

			unsigned int getLeft()
			{
				return left;
			};

			unsigned int getRight()
			{
				return right;
			};

			unsigned int getSpacer()
			{
				return spacer;
			};

			virtual void updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area) = 0;
			virtual Util::Size getPreferedSize()=0;
			virtual void testPaint(){};
		public:
			~Layout(void)
			{};
		};
	}
}