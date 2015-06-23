#pragma once

#include <vector>
#include "BoundingBox.h"
#include "FastDelegate.h"

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
			bool isHover;
			bool isEnable;
			bool isVisible;
		private:
			int layoutProperty;
		public:
			Component(void):isEnable(true),isVisible(true),isHover(false),layoutProperty(0)
			{};

			virtual void paint()
			{};

			void setLayoutProperty(int _layoutProperty)
			{
				layoutProperty=_layoutProperty;
			};

			int getLayoutProperty()
			{
				return layoutProperty;	
			};

			void setLocation(int x,int y)
			{
				position.x=x;
				position.y=y;
			};

			void setSize(unsigned int width,unsigned int height)
			{
				size.width=width;
				size.height=height;
			};

			void processMouseClick(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseClickHandlerList.begin();iter<mouseClickHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			void processMousePressed(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mousePressedHandlerList.begin();iter<mousePressedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			void processMouseReleased(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseReleasedHandlerList.begin();iter<mouseReleasedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			void processMouseEntered(const Event::MouseEvent &e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseEnteredHandlerList.begin();iter<mouseEnteredHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			void processMouseExited(const Event::MouseEvent& e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseExitedHandlerList.begin();iter<mouseExitedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			void processMouseMoved(const Event::MouseEvent& e)
			{
				std::vector<MouseDelegate>::iterator iter;
				for(iter=mouseMovedHandlerList.begin();iter<mouseMovedHandlerList.end();++iter)
				{
					(*iter)(e);
				}
			};

			virtual Util::Size getPreferedSize() =0;
			virtual void pack(){};
		public:
			typedef fastdelegate::FastDelegate1<const Event::MouseEvent &> MouseDelegate;
			std::vector<MouseDelegate> mouseClickHandlerList;
			std::vector<MouseDelegate> mousePressedHandlerList;
			std::vector<MouseDelegate> mouseReleasedHandlerList;
			std::vector<MouseDelegate> mouseEnteredHandlerList;
			std::vector<MouseDelegate> mouseExitedHandlerList;
			std::vector<MouseDelegate> mouseMovedHandlerList;

		public:
			~Component(void)
			{
				mouseClickHandlerList.clear();
				mousePressedHandlerList.clear();
				mouseReleasedHandlerList.clear();
				mouseEnteredHandlerList.clear();
				mouseExitedHandlerList.clear();
				mouseMovedHandlerList.clear();
			};
		};
	}
}