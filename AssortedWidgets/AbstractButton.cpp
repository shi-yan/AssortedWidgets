#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        AbstractButton::AbstractButton(unsigned int top, unsigned int bottom, unsigned int left, unsigned int right, enum Status status)
            :m_top(top),
              m_bottom(bottom),
              m_left(left),
              m_right(right),
              m_status(status)
		{
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			MouseDelegate mEntered;
			mEntered.bind(this,&AbstractButton::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&AbstractButton::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mPressed;
			mPressed.bind(this,&AbstractButton::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&AbstractButton::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);
		}

		void AbstractButton::mousePressed(const Event::MouseEvent &e)
		{
            m_status=pressed;
        }
		
		void AbstractButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
            m_status=hover;
        }

		void AbstractButton::mouseReleased(const Event::MouseEvent &e)
		{
            m_status=hover;
        }

		void AbstractButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
            m_status=normal;
        }

		AbstractButton::~AbstractButton(void)
		{
		}
	}
}
