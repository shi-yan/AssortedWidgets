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
            DialogTittleBar m_tittleBar;
            DialogUpLeft m_borderUpLeft;
            DialogUpRight m_borderUpRight;
            DialogUp m_borderUp;
            DialogLeft m_borderLeft;
            DialogRight m_borderRight;
            DialogBottomLeft m_borderBottomLeft;
            DialogBottom m_borderBottom;
            DialogBottomRight m_borderBottomRight;

            bool m_dragable;
            bool m_resizable;
            bool m_active;
            enum ShowType m_showType;
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            Util::Position m_contentPosition;
            Util::Size m_contentSize;
			
		public:
            void setShowType(enum ShowType _showType)
			{
                m_showType=_showType;
            }
            enum ShowType getShowType() const
			{
                return m_showType;
            }
			void setDragable(bool _dragable)
			{
                m_dragable=_dragable;
            }

			void setActive(bool _active)
			{
                m_active=_active;
            }

            bool isActive() const
			{
                return m_active;
            }

			void setResizable(bool _resizable)
			{
                m_resizable=_resizable;
            }

			void Close();

			void pack();
			Dialog(std::string &tittle,int x,int y,unsigned int width,unsigned int height);
			Dialog(char *tittle,int x,int y,unsigned int width,unsigned int height);
			Util::Size getPreferedSize()
			{
				//return Theme::ThemeEngine::getSingleton().getTheme().getDialogPreferedSize(this);
                Util::Size result(m_tittleBar.getPreferedSize());
                result.width+=m_left+m_right;
                result.height+=m_top+m_bottom;
				return result;
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
                    Theme::ThemeEngine::getSingleton().getTheme().scissorBegin(m_contentPosition,m_contentSize);
					(*iter)->paint();
					Theme::ThemeEngine::getSingleton().getTheme().scissorEnd();
				}
            }
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDialog(this);
                Util::Position p(m_position);
                Util::Graphics::getSingleton().pushPosition(p);
                m_tittleBar.paint();
				//layout->testPaint();
				paintChild();
				Util::Graphics::getSingleton().popPosition();
            }
		public:
			~Dialog(void);
		};
	}
}
