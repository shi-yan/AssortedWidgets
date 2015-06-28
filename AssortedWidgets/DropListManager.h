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
            int m_currentX;
            int m_currentY;
            Widgets::DropList *m_currentDropped;
            Util::Size m_size;
            Util::Position m_position;

		public:
            bool m_isHover;

			void shrinkBack();
			Widgets::DropList* getDropped()
			{
                return m_currentDropped;
            }

			bool isIn(int mx,int my)
			{
                if((mx>m_position.x && mx<static_cast<int>(m_position.x+m_size.m_width))&&(my>m_position.y&&my<static_cast<float>(m_position.y+m_size.m_height)))
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
                m_currentX=_currentX;
                m_currentY=_currentY;
            }

			void setDropped(Widgets::DropList *_currentDropped,int rx,int ry);

			void paint();

            bool isDropped() const
			{
                return m_currentDropped != NULL;
			}

			static DropListManager & getSingleton()
			{
				static DropListManager obj;
				return obj;
            }
		private:
			DropListManager(void);
			~DropListManager(void);
		};
	}
}
