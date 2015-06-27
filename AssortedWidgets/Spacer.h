#pragma once
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        class Spacer: public Element
		{
		public:
			enum Type
			{
				Fit,
				Horizontal,
				Vertical
			};
		private:
            enum Type m_type;
		public:
            enum Type getType() const
			{
                return m_type;
            }
            Util::Size getPreferedSize()
			{
				return Util::Size(2,2);
            }
            void paint(){}
            Spacer(enum Type _type);
		public:
			~Spacer(void);
		};
	}
}
