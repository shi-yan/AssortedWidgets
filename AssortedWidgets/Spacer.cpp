#include "Spacer.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        Spacer::Spacer(enum Type _type)
            :Element(),
              m_type(_type)
		{
            if(m_type==Horizontal)
			{
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
			}
            else if(m_type==Vertical)
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
