#pragma once
#include <GL/gl.h>
#include <GL/glu.h>
#include "DragAble.h"
#include "Graphics.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Dialog;

		class DialogRight:public DragAble
		{
		private:
			Dialog *parent;
		public:
			DialogRight(int x,int y,unsigned int width,unsigned int height);
			void setParent(Dialog *_parent)
			{
				parent=_parent;
			};
			Util::Size getPreferedSize()
			{
				return size;
			};
			void paint()
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(0,0,255);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<GLfloat>(origin.x+position.x),static_cast<GLfloat>(origin.y+position.y));
				glVertex2f(static_cast<GLfloat>(origin.x+position.x+size.width),static_cast<GLfloat>(origin.y+position.y));
				glVertex2f(static_cast<GLfloat>(origin.x+position.x+size.width),static_cast<GLfloat>(origin.y+position.y+size.height));
				glVertex2f(static_cast<GLfloat>(origin.x+position.x),static_cast<GLfloat>(origin.y+position.y+size.height));
				glEnd();
			};
			void dragReleased(const Event::MouseEvent &e);
			void dragMoved(int offsetX,int offsetY);
		public:
			~DialogRight(void);
		};
	}
}
