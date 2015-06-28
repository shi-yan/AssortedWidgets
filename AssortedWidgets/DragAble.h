#pragma once
#include "Component.h"
#include "MouseEvent.h"
#include "SelectionManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DragAble:virtual public Component
		{
		private:
			Manager::SelectionManager *selectionManager;
		public:
			DragAble(void);
			void setSelectionManager(Manager::SelectionManager *_selectionManager)
			{
				selectionManager=_selectionManager;
            }
			void dragPressed(const Event::MouseEvent &e);
			virtual void dragReleased(const Event::MouseEvent &e)=0;
			virtual void dragMoved(int offsetX,int offsetY)=0;
		public:
			~DragAble(void);
		};
	}
}
