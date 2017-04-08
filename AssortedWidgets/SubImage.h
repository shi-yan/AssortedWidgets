#pragma once
#ifdef __APPLE__
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GLES2/gl2.h>

#endif

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
            {

            }
            void paint(const float x1,const float y1,const float x2,const float y2) const;

		public:
			~SubImage(void)
			{
            }
		};
	}
}
