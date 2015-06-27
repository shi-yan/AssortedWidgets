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
			GLfloat UpLeftX;
			GLfloat UpLeftY;
			GLfloat BottomRightX;
			GLfloat BottomRightY;
			GLuint textureID;
		public:
			SubImage(GLfloat _UpLeftX,GLfloat _UpLeftY,GLfloat _BottomRightX,GLfloat _BottomRightY,GLuint _textureID):UpLeftX(_UpLeftX),UpLeftY(_UpLeftY),BottomRightX(_BottomRightX),BottomRightY(_BottomRightY),textureID(_textureID)
			{};
			void paint(const float x1,const float y1,const float x2,const float y2) const
			{
				glColor3ub(255,255,255);
				glBindTexture(GL_TEXTURE_2D,textureID);
				glBegin(GL_QUADS);
				glTexCoord2f(UpLeftX,UpLeftY);
				glVertex2f(x1,y1);
				glTexCoord2f(UpLeftX,BottomRightY);
				glVertex2f(x1,y2);
				glTexCoord2f(BottomRightX,BottomRightY);
				glVertex2f(x2,y2);
				glTexCoord2f(BottomRightX,UpLeftY);
				glVertex2f(x2,y1);
				glEnd();
			};
		public:
			~SubImage(void)
			{
				glDeleteTextures(1,&textureID);
			};
		};
	}
}
