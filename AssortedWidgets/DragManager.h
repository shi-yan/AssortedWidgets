#pragma once
#include "DragAble.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		class DragManager
		{
		private:
			Widgets::DragAble *componentOnDrag;
			int oldX;
			int oldY;
			int preX;
			int preY;
		private:
			DragManager(void):componentOnDrag(0),oldX(0),oldY(0)
			{};
		public:
			int currentX;
			int currentY;
			static DragManager& getSingleton()
			{
				static DragManager obj;
				return obj;
			};
			void setCurrent(int _currentX,int _currentY)
			{
				currentX=_currentX;
				currentY=_currentY;
			};
			void dragBegin(int _oldX,int _oldY,Widgets::DragAble *component)
			{
				oldX=_oldX;
				oldY=_oldY;
				preX=currentX;
				preY=currentY;
				componentOnDrag=component;
			};

			void dragEnd()
			{
				oldX=0;
				oldY=0;
				preX=0;
				preY=0;
				componentOnDrag=0;
			}

			bool isOnDrag()
			{
				return componentOnDrag!=0;
			};

			void processDrag(int x,int y)
			{
				if(isOnDrag())				
				{
					componentOnDrag->dragMoved(x-preX,y-preY);
					preX=x;
					preY=y;
				}
			};

			Widgets::DragAble* getOnDragComponent()
			{
				return componentOnDrag;
			};

			int getOldX()
			{
				return oldX;
			};

			int getOldY()
			{
				return oldY;
			};
		private:
			~DragManager(void){};
		};
	}
}