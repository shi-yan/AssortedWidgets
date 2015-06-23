#pragma once
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Layout
	{
		class FlowLayout:public Layout
		{
		public:
			FlowLayout(void):Layout()
			{};
			FlowLayout(unsigned int spacer):Layout(spacer)
			{};
			FlowLayout(unsigned int top,unsigned int bottom,unsigned int left,unsigned int right,unsigned int spacer):Layout(top,bottom,left,right,spacer)
			{}
			void updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area);
			Util::Size getPreferedSize();

		public:
			~FlowLayout(void);
		};
	}
}