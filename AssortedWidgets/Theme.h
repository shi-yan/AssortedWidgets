#pragma once
#include "Component.h"

namespace AssortedWidgets
{
	namespace Theme
	{
		class Theme
		{
		public:
            virtual void paint(AssortedWidgets::Widgets::Component *component) const =0;
            virtual void getPreferedSize(AssortedWidgets::Widgets::Component *component) const =0;
		public:
			Theme(void);
		public:
			~Theme(void);
		};
	}
}
