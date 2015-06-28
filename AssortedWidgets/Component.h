#pragma once

#include <vector>
#include "BoundingBox.h"
#include <functional>

#define MOUSE_DELEGATE(func) std::bind(&func, this, std::placeholders::_1)

namespace AssortedWidgets
{
	namespace Event
	{
		class KeyEvent;
		class MouseEvent;
	}

	namespace Widgets
	{
		class Component: public Util::BoundingBox
		{
		public:
            bool m_isHover;
            bool m_isEnable;
            bool m_isVisible;
		private:
            int m_layoutProperty;
		public:
            Component(void)
                :m_isEnable(true),
                  m_isVisible(true),
                  m_isHover(false),
                  m_layoutProperty(0)
            {}

			virtual void paint()
            {}

			void setLayoutProperty(int _layoutProperty)
			{
                m_layoutProperty=_layoutProperty;
            }

            int getLayoutProperty() const
			{
                return m_layoutProperty;
            }

			void setLocation(int x,int y)
			{
                m_position.x=x;
                m_position.y=y;
            }

			void setSize(unsigned int width,unsigned int height)
			{
                m_size.m_width=width;
                m_size.m_height=height;
            }

			void processMouseClick(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseClickHandlerList.begin();iter<mouseClickHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

			void processMousePressed(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mousePressedHandlerList.begin();iter<mousePressedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

			void processMouseReleased(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseReleasedHandlerList.begin();iter<mouseReleasedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

			void processMouseEntered(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseEnteredHandlerList.begin();iter<mouseEnteredHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

			void processMouseExited(const Event::MouseEvent& e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseExitedHandlerList.begin();iter<mouseExitedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

			void processMouseMoved(const Event::MouseEvent& e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseMovedHandlerList.begin();iter<mouseMovedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
            }

            //not const for now
            virtual Util::Size getPreferedSize() = 0;
            virtual void pack(){}
		public:
            typedef std::function<void(const Event::MouseEvent &)> MouseDelegate;
			std::vector<MouseDelegate> mouseClickHandlerList;
			std::vector<MouseDelegate> mousePressedHandlerList;
			std::vector<MouseDelegate> mouseReleasedHandlerList;
			std::vector<MouseDelegate> mouseEnteredHandlerList;
			std::vector<MouseDelegate> mouseExitedHandlerList;
			std::vector<MouseDelegate> mouseMovedHandlerList;

		public:
            virtual ~Component(void)
			{
				mouseClickHandlerList.clear();
				mousePressedHandlerList.clear();
				mouseReleasedHandlerList.clear();
				mouseEnteredHandlerList.clear();
				mouseExitedHandlerList.clear();
				mouseMovedHandlerList.clear();
            }
		};
	}
}
