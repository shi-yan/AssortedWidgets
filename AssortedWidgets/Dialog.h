#pragma once
#include "ContainerElement.h"
#include "DialogTittleBar.h"
#include "DialogUpLeft.h"
#include "DialogUpRight.h"
#include "DialogUp.h"
#include "DialogLeft.h"
#include "DialogRight.h"
#include "DialogBottomLeft.h"
#include "DialogBottom.h"
#include "DialogBottomRight.h"
#include "ThemeEngine.h"
#include "Graphics.h"
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Dialog:public Container
		{
		public:
			enum ShowType
			{
				None,
				Modal,
				Modeless
			};
		private:
			DialogTittleBar tittleBar;

			DialogUpLeft borderUpLeft;
			DialogUpRight borderUpRight;
			DialogUp borderUp;
			DialogLeft borderLeft;
			DialogRight borderRight;
			DialogBottomLeft borderBottomLeft;
			DialogBottom borderBottom;
			DialogBottomRight borderBottomRight;

			bool dragable;
			bool resizable;

			bool active;

			int showType;

			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;

			Util::Position contentPosition;
			Util::Size contentSize;
			
		public:
			void setShowType(int _showType)
			{
				showType=_showType;
			};
			int getShowType()
			{
				return showType;
			};
			void setDragable(bool _dragable)
			{
				dragable=_dragable;
			};

			void setActive(bool _active)
			{
				active=_active;
			};

			bool isActive()
			{
				return active;
			};

			void setResizable(bool _resizable)
			{
				resizable=_resizable;
			};

			void Close();

			void pack();
			Dialog(std::string &tittle,int x,int y,unsigned int width,unsigned int height);
			Dialog(char *tittle,int x,int y,unsigned int width,unsigned int height);
			Util::Size getPreferedSize()
			{
				//return Theme::ThemeEngine::getSingleton().getTheme().getDialogPreferedSize(this);
				Util::Size result(tittleBar.getPreferedSize());
				result.width+=left+right;
				result.height+=top+bottom;
				return result;
			};
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
				
			};
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDialog(this);
				Util::Graphics::getSingleton().pushPosition(Util::Position(position));
				tittleBar.paint();
				//layout->testPaint();
				paintChild();
				Util::Graphics::getSingleton().popPosition();
			};
		public:
			~Dialog(void);
		};
	}
}