#include "DragAble.h"
#include "DragManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DragAble::DragAble(void):selectionManager(0)
        {
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(DragAble::dragPressed));
		}

		void DragAble::dragPressed(const Event::MouseEvent &e)
		{
            Manager::DragManager::getSingleton().dragBegin(m_position.x,m_position.y,this);
		}

		DragAble::~DragAble(void)
		{
		}
	}
}
