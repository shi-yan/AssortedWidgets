#pragma once
#include <GL/gl.h>
#include <GL/glu.h>

namespace AssortedWidgets
{
	namespace Theme
	{
		class SubImage
		{
		private:
            GLfloat m_UpLeftX;
            GLfloat m_UpLeftY;
            GLfloat m_BottomRightX;
            GLfloat m_BottomRightY;
            GLuint m_textureID;
		public:
            SubImage(GLfloat _UpLeftX, GLfloat _UpLeftY, GLfloat _BottomRightX, GLfloat _BottomRightY, GLuint _textureID)
                :m_UpLeftX(_UpLeftX),
                  m_UpLeftY(_UpLeftY),
                  m_BottomRightX(_BottomRightX),
                  m_BottomRightY(_BottomRightY),
                  m_textureID(_textureID)
            {}
			void paint(const float x1,const float y1,const float x2,const float y2) const
			{
				glColor3ub(255,255,255);
                glBindTexture(GL_TEXTURE_2D, m_textureID);
				glBegin(GL_QUADS);
                glTexCoord2f(m_UpLeftX, m_UpLeftY);
				glVertex2f(x1,y1);
                glTexCoord2f(m_UpLeftX, m_BottomRightY);
				glVertex2f(x1,y2);
                glTexCoord2f(m_BottomRightX, m_BottomRightY);
				glVertex2f(x2,y2);
                glTexCoord2f(m_BottomRightX, m_UpLeftY);
				glVertex2f(x2,y1);
				glEnd();
            }
		public:
			~SubImage(void)
			{
                glDeleteTextures(1,&m_textureID);
			};
		};
	}
}
