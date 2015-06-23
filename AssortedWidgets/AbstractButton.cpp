#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		AbstractButton::AbstractButton(void):top(4),bottom(4),left(8),right(8),status(normal)
		{
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			//isHover=false;
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

		AbstractButton::AbstractButton(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right):top(_top),bottom(_bottom),left(_left),right(_right),status(normal)
		{
			horizontalStyle=Element::Fit;
			verticalStyle=Element::Fit;
			//isHover=false;
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

		AbstractButton::AbstractButton(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right,int _status):top(_top),bottom(_bottom),left(_left),right(_right),status(_status)
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
			status=pressed;
		};
		
		void AbstractButton::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			status=hover;

		};

		void AbstractButton::mouseReleased(const Event::MouseEvent &e)
		{
			status=hover;
		};

		void AbstractButton::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			status=normal;
		};

		AbstractButton::~AbstractButton(void)
		{
		}
	}
}