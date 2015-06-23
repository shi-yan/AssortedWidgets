#pragma once
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Spacer:public Element
		{
		public:
			enum Type
			{
				Fit,
				Horizontal,
				Vertical
			};
		private:
			int type;
		public:
			int getType()
			{
				return type;
			};
			Util::Size getPreferedSize()
			{
				return Util::Size(2,2);
			};
			void paint(){};
			Spacer(int _type);
		public:
			~Spacer(void);
		};
	}
}