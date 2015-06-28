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

		class DialogBottomRight:public DragAble
		{
		private:
            Dialog *m_parent;
		public:
			DialogBottomRight(int x,int y,unsigned int width,unsigned int height);
			void setParent(Dialog *_parent)
			{
                m_parent=_parent;
            }
			Util::Size getPreferedSize()
			{
                return m_size;
            }
			void paint()
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(255,0,0);
				glBegin(GL_QUADS);
                glVertex2f(static_cast<GLfloat>(origin.x+m_position.x),static_cast<GLfloat>(origin.y+m_position.y));
                glVertex2f(static_cast<GLfloat>(origin.x+m_position.x+m_size.m_width),static_cast<GLfloat>(origin.y+m_position.y));
                glVertex2f(static_cast<GLfloat>(origin.x+m_position.x+m_size.m_width),static_cast<GLfloat>(origin.y+m_position.y+m_size.m_height));
                glVertex2f(static_cast<GLfloat>(origin.x+m_position.x),static_cast<GLfloat>(origin.y+m_position.y+m_size.m_height));
				glEnd();
            }
			void dragReleased(const Event::MouseEvent &e);
			void dragMoved(int offsetX,int offsetY);
		public:
			~DialogBottomRight(void);
		};
	}
}
