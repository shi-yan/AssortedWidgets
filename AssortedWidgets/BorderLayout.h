#pragma once
#include <windows.h>
#include <gl/gl.h>
#include <gl/glu.h>
#include "Graphics.h"
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Layout
	{
		class BorderLayout:public Layout
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
			int eastFormat;
			int southFormat;
			int westFormat;
			int northFormat;
			int centerFormat;

			int eastHAlignment;
			int southHAlignment;
			int westHAlignment;
			int northHAlignment;
			int centerHAlignment;

			int eastVAlignment;
			int southVAlignment;
			int westVAlignment;
			int northVAlignment;
			int centerVAlignment;

			float testNorthX;
			float testNorthY;
			float testNorthWidth;
			float testNorthHeight;

		public:
			void testPaint()
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(0,0,255);
				glBegin(GL_QUADS);
				glVertex2f(origin.x+testNorthX,origin.y+testNorthY);
				glVertex2f(origin.x+testNorthX+testNorthWidth,origin.y+testNorthY);
				glVertex2f(origin.x+testNorthX+testNorthWidth,origin.y+testNorthY+testNorthHeight);
				glVertex2f(origin.x+testNorthX,origin.y+testNorthY+testNorthHeight);
				glEnd();
			};

			void setEastHAlignment(int _eastHAlignment)
			{
				eastHAlignment=_eastHAlignment;
			};

			void setSouthHAlignment(int _southHAlignment)
			{
				southHAlignment=_southHAlignment;
			};

			void setWestHAlignment(int _westHAlignment)
			{
				westHAlignment=_westHAlignment;
			};

			void setNorthHAlignment(int _northHAlignment)
			{
				northHAlignment=_northHAlignment;
			};

			void setCenterHAlignment(int _centerHAlignment)
			{
				centerHAlignment=_centerHAlignment;
			};

			void setEastVAlignment(int _eastVAlignment)
			{
				eastVAlignment=_eastVAlignment;
			};

			void setSouthVAlignment(int _southVAlignment)
			{
				southVAlignment=_southVAlignment;
			};

			void setWestVAlignment(int _westVAlignment)
			{
				westVAlignment=_westVAlignment;
			};

			void setNorthVAlignment(int _northVAlignment)
			{
				northVAlignment=_northVAlignment;
			};

			void setCenterVAlignment(int _centerVAlignment)
			{
				centerVAlignment=_centerVAlignment;
			};

			void setEastFormat(int _eastFormat)
			{
				eastFormat=_eastFormat;
			};

			void setSouthFormat(int _southFormat)
			{
				southFormat=_southFormat;
			};

			void setNorthFormat(int _northFormat)
			{
				northFormat=_northFormat;
			};

			void setWestFormat(int _westFormat)
			{
				westFormat=_westFormat;
			};

			void setCenterFormat(int _centerFormat)
			{
				centerFormat=_centerFormat;
			};

			void updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area);
			Util::Size getPreferedSize();
			BorderLayout(void)
				:Layout(),eastFormat(horizontal),southFormat(horizontal),westFormat(horizontal),northFormat(horizontal),centerFormat(horizontal),eastHAlignment(HLeft),southHAlignment(HLeft),westHAlignment(HLeft),northHAlignment(HLeft),centerHAlignment(HLeft),eastVAlignment(VCenter),southVAlignment(VCenter),westVAlignment(VCenter),northVAlignment(VCenter),centerVAlignment(VCenter)
			{
				testNorthX=0;
				testNorthY=0;
				testNorthWidth=0;
				testNorthHeight=0;
			};
			BorderLayout(unsigned int _spacer)
				:Layout(_spacer),eastFormat(horizontal),southFormat(horizontal),westFormat(horizontal),northFormat(horizontal),centerFormat(horizontal),eastHAlignment(HLeft),southHAlignment(HLeft),westHAlignment(HLeft),northHAlignment(HLeft),centerHAlignment(HLeft),eastVAlignment(VCenter),southVAlignment(VCenter),westVAlignment(VCenter),northVAlignment(VCenter),centerVAlignment(VCenter)
			{
				testNorthX=0;
				testNorthY=0;
				testNorthWidth=0;
				testNorthHeight=0;
			};
			BorderLayout(unsigned int _top,unsigned int _bottom,unsigned int _left,unsigned int _right,unsigned int _spacer)
				:Layout(_top,_bottom,_left,_right,_spacer),eastFormat(horizontal),southFormat(horizontal),westFormat(horizontal),northFormat(horizontal),centerFormat(horizontal),eastHAlignment(HLeft),southHAlignment(HLeft),westHAlignment(HLeft),northHAlignment(HLeft),centerHAlignment(HLeft),eastVAlignment(VCenter),southVAlignment(VCenter),westVAlignment(VCenter),northVAlignment(VCenter),centerVAlignment(VCenter)
			{
				testNorthX=0;
				testNorthY=0;
				testNorthWidth=0;
				testNorthHeight=0;
			};
					
		private:
			unsigned int getPreferedHeight(std::vector<Widgets::Element*> &list,int format);
			unsigned int getPreferedWidth(std::vector<Widgets::Element*> &list,int format);
			void orderComponents(std::vector<Widgets::Element*> &list,int HAlignment,int VAlignment,int format,Util::Position &origin,Util::Size &area);
		public:
			~BorderLayout(void);
		};
	}
}