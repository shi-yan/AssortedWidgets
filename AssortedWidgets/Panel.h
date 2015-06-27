#pragma once
#include "ContainerElement.h"
#include "ThemeEngine.h"
#include "Graphics.h"
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        class Panel: public Element, public Container
		{
		private:
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;

			Util::Position contentPosition;
			Util::Size contentSize;
		public:
			void pack();
			Panel(void);

			Util::Size getPreferedSize()
			{
				return Util::Size(10,10);
            }

            void mousePressed(const Event::MouseEvent &e);
			
			void mouseReleased(const Event::MouseEvent &e);
			void mouseEntered(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);
			void paintChild()
			{
				
				std::vector<Element*>::iterator iter;
				for(iter=childList.begin();iter<childList.end();++iter)
				{
					Theme::ThemeEngine::getSingleton().getTheme().scissorBegin(contentPosition,contentSize);
					(*iter)->paint();
					Theme::ThemeEngine::getSingleton().getTheme().scissorEnd();
				}
				
            }
			void paint()
			{
                Util::Position p(position);
                Util::Graphics::getSingleton().pushPosition(p);
				paintChild();
				Util::Graphics::getSingleton().popPosition();
            }
		public:
			~Panel(void);
		};
	}
}
