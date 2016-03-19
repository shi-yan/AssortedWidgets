#pragma once
#ifdef __APPLE__
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GL/gl.h>
#include <GL/glu.h>
#endif
#include "Graphics.h"
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Layout
	{
        class BorderLayout: public Layout
		{
		public:
			enum Format
			{
				horizontal,
				vertical
			};

			enum HorizontalAlignment
			{
				HLeft,
				HCenter,
				HRight,
			};

			enum VerticalAlignment
			{
				VTop,
				VCenter,
				VBottom
			};

			enum Area
			{
				East,
				South,
				West,
				North,
				Center
			};

		private:
            int m_eastFormat;
            int m_southFormat;
            int m_westFormat;
            int m_northFormat;
            int m_centerFormat;

            int m_eastHAlignment;
            int m_southHAlignment;
            int m_westHAlignment;
            int m_northHAlignment;
            int m_centerHAlignment;

            int m_eastVAlignment;
            int m_southVAlignment;
            int m_westVAlignment;
            int m_northVAlignment;
            int m_centerVAlignment;

            float m_testNorthX;
            float m_testNorthY;
            float m_testNorthWidth;
            float m_testNorthHeight;

		public:
			void testPaint()
			{
                Util::Position origin = Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(0,0,255);
				glBegin(GL_QUADS);
                glVertex2f(origin.x + m_testNorthX, origin.y + m_testNorthY);
                glVertex2f(origin.x + m_testNorthX + m_testNorthWidth, origin.y + m_testNorthY);
                glVertex2f(origin.x + m_testNorthX + m_testNorthWidth, origin.y + m_testNorthY + m_testNorthHeight);
                glVertex2f(origin.x + m_testNorthX, origin.y + m_testNorthY + m_testNorthHeight);
				glEnd();
            }

            void setEastHAlignment(int eastHAlignment)
			{
                m_eastHAlignment = eastHAlignment;
            }

            void setSouthHAlignment(int southHAlignment)
			{
                m_southHAlignment = southHAlignment;
            }

            void setWestHAlignment(int westHAlignment)
			{
                m_westHAlignment = westHAlignment;
            }

            void setNorthHAlignment(int northHAlignment)
			{
                m_northHAlignment = northHAlignment;
            }

            void setCenterHAlignment(int centerHAlignment)
			{
                m_centerHAlignment = centerHAlignment;
            }

            void setEastVAlignment(int eastVAlignment)
			{
                m_eastVAlignment = eastVAlignment;
            }

            void setSouthVAlignment(int southVAlignment)
			{
                m_southVAlignment = southVAlignment;
            }

            void setWestVAlignment(int westVAlignment)
			{
                m_westVAlignment = westVAlignment;
            }

            void setNorthVAlignment(int northVAlignment)
			{
                m_northVAlignment = northVAlignment;
            }

            void setCenterVAlignment(int centerVAlignment)
			{
                m_centerVAlignment = centerVAlignment;
            }

            void setEastFormat(int eastFormat)
			{
                m_eastFormat = eastFormat;
            }

            void setSouthFormat(int southFormat)
			{
                m_southFormat = southFormat;
            }

            void setNorthFormat(int northFormat)
			{
                m_northFormat = northFormat;
            }

            void setWestFormat(int westFormat)
			{
                m_westFormat = westFormat;
            }

            void setCenterFormat(int centerFormat)
			{
                m_centerFormat = centerFormat;
            }

            void updateLayout(std::vector<Widgets::Element *> &componentList, Util::Position &origin, Util::Size &area);
            Util::Size getPreferedSize() const;

            BorderLayout(unsigned int spacer = 2, unsigned int top = 0, unsigned int bottom = 0, unsigned int left = 0, unsigned int right = 0)
                :Layout(spacer, top, bottom, left, right),
                  m_eastFormat(horizontal),
                  m_southFormat(horizontal),
                  m_westFormat(horizontal),
                  m_northFormat(horizontal),
                  m_centerFormat(horizontal),
                  m_eastHAlignment(HLeft),
                  m_southHAlignment(HLeft),
                  m_westHAlignment(HLeft),
                  m_northHAlignment(HLeft),
                  m_centerHAlignment(HLeft),
                  m_eastVAlignment(VCenter),
                  m_southVAlignment(VCenter),
                  m_westVAlignment(VCenter),
                  m_northVAlignment(VCenter),
                  m_centerVAlignment(VCenter),
                  m_testNorthX(0),
                  m_testNorthY(0),
                  m_testNorthWidth(0),
                  m_testNorthHeight(0)
            {
            }

		private:
			unsigned int getPreferedHeight(std::vector<Widgets::Element*> &list,int format);
			unsigned int getPreferedWidth(std::vector<Widgets::Element*> &list,int format);
			void orderComponents(std::vector<Widgets::Element*> &list,int HAlignment,int VAlignment,int format,Util::Position &origin,Util::Size &area);
		public:
			~BorderLayout(void);
		};
	}
}
