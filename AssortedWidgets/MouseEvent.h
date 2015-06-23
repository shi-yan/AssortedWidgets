#pragma once

#include "Event.h"
#include "Component.h"

namespace AssortedWidgets
{
	namespace Event
	{
		class MouseEvent:public Event
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

			MouseEvent(Widgets::Component* _source, int _type, int _x, int _y, int _mouseButton):Event(_source,_type),mouseX(_x),mouseY(_y),mouseButton(_mouseButton)
			{};

			int getButton()
			{
				return mouseButton;
			};

			int getX() const
			{
				return mouseX;
			};

			int getY() const
			{
				return mouseY;
			};
		private:
			int mouseX;
			int mouseY;
			int mouseButton;
		public:
			~MouseEvent(void){};
		};
	}
}
