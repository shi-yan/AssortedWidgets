#pragma once
#include <string>
#include "ContainerElement.h"
#include "ThemeEngine.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        class AbstractButton: public Element
		{
		public:
			enum Status
			{
				normal,
				hover,
				pressed
			};

		private:
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            enum Status m_status;

		public:
            AbstractButton(unsigned int top = 4, unsigned int bottom = 4, unsigned int left = 8, unsigned int right = 8, enum Status status = normal);
			
            unsigned int getTop() const
			{
                return m_top;
            }

            unsigned int getBottom() const
			{
                return m_bottom;
            }

            unsigned int getRight() const
			{
                return m_right;
            }

            unsigned int getLeft() const
			{
                return m_left;
            }

            enum Status getStatus() const
			{
                return m_status;
            }

			void mousePressed(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);

		public:
			~AbstractButton(void);
        };
	}
}
