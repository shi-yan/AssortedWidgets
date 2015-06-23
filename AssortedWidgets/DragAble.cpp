#include "DragAble.h"
#include "DragManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DragAble::DragAble(void):selectionManager(0)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&DragAble::dragPressed);
			mousePressedHandlerList.push_back(mPressed);
		}

		void DragAble::dragPressed(const Event::MouseEvent &e)
		{
			Manager::DragManager::getSingleton().dragBegin(position.x,position.y,this);
		}

		DragAble::~DragAble(void)
		{
		}
	}
}