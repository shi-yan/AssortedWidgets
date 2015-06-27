#pragma once

#include "Event.h"
#include "Component.h"

namespace AssortedWidgets
{
	namespace Event
	{
        class MouseEvent: public Event
		{
		public:
			enum MouseEventTypes
			{
				MOUSE_PRESSED,
				MOUSE_RELEASED,
				MOUSE_CLICKED,
				MOUSE_EXITED,
				MOUSE_DRAGGED,
				MOUSE_ENTERED,
				MOUSE_MOTION
			};

			enum MouseButtons
			{
				MOUSE_LEFT,
				MOUSE_RIGHT,
				MOUSE_MIDDLE,
				MOUSE_SCROLL_UP,
				MOUSE_SCROLL_DOWN
			};

            MouseEvent(Widgets::Component* _source, int _type, int _x, int _y, int _mouseButton)
                :Event(_source,_type),
                  m_mouseX(_x),
                  m_mouseY(_y),
                  m_mouseButton(_mouseButton)
            {}

            int getButton() const
			{
                return m_mouseButton;
            }

			int getX() const
			{
                return m_mouseX;
            }

			int getY() const
			{
                return m_mouseY;
            }
		private:
            int m_mouseX;
            int m_mouseY;
            int m_mouseButton;
		public:
            ~MouseEvent(void){}
		};
	}
}
