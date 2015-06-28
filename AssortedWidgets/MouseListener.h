#pragma once

#include <vector>
#include "EventListener.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Event
	{
		class MouseListener:public EventListener
		{
		public:
			virtual void mouseClicked(const MouseEvent &e) = 0;
			virtual void mousePressed(const MouseEvent &e) = 0;
			virtual void mouseEntered(const MouseEvent &e) = 0;
			virtual void mouseExited(const MouseEvent &e) = 0;
			virtual void mouseReleased(const MouseEvent &e) = 0;
			virtual void mouseDragged(const MouseEvent &e) = 0;
			virtual void mouseMotion(const MouseEvent &e) = 0;
		public:
            MouseListener(void){}
		public:
            ~MouseListener(void){}
		};

		class MouseAdapter: public MouseListener
		{
		public:
            virtual void mouseClicked(const MouseEvent &e){}
            virtual void mousePressed(const MouseEvent &e){}
            virtual void mouseEntered(const MouseEvent &e){}
            virtual void mouseExited(const MouseEvent &e){}
            virtual void mouseReleased(const MouseEvent &e){}
            virtual void mouseDragged(const MouseEvent &e){}
            virtual void mouseMotion(const MouseEvent &e){}
		public:
            MouseAdapter(void){}
		public:
            ~MouseAdapter(void){}
		};
	}
}
