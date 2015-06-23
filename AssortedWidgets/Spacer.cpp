#include "Spacer.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		Spacer::Spacer(int _type):type(_type)
		{
			if(type==Horizontal)
			{
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
			}
			else if(type==Vertical)
			{
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);				
			}
			else
			{
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Fit);							
			}
		}

		Spacer::~Spacer(void)
		{
		}
	}
}