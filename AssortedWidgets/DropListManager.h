#pragma once
#include "ThemeEngine.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DropList;
	}
	namespace Manager
	{
		class DropListManager
		{
		private:
			int currentX;
			int currentY;
			
			Widgets::DropList *currentDropped;
			Util::Size size;
			Util::Position position;
		public:
            bool isHover;
			void shrinkBack();
			Widgets::DropList* getDropped()
			{
				return currentDropped;
            }

			bool isIn(int mx,int my)
			{
				if((mx>position.x && mx<static_cast<int>(position.x+size.width))&&(my>position.y&&my<static_cast<float>(position.y+size.height)))
				{
					return true;
				}
				else
				{
					return false;
				}
			}

			void importMouseMotion(Event::MouseEvent &e);
			void importMouseEntered(Event::MouseEvent &e);
			void importMouseExited(Event::MouseEvent &e);
			void importMousePressed(Event::MouseEvent &e);

			void setCurrent(int _currentX,int _currentY)
			{
				currentX=_currentX;
				currentY=_currentY;
			};

			void setDropped(Widgets::DropList *_currentDropped,int rx,int ry);

			void paint();


			bool isDropped()
			{
				return currentDropped!=0;
			}

			static DropListManager & getSingleton()
			{
				static DropListManager obj;
				return obj;
			};
		private:
			DropListManager(void);
			~DropListManager(void);
		};
	}
}
