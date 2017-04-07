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

            static GLuint m_vertShader;
            static GLuint m_fragShader;
            static GLuint m_shaderProgram;
            static GLint m_screenSizeUniform;
            static GLint m_textureUniform;

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

            static void init(unsigned int width, unsigned int height);
		public:
			~SubImage(void)
			{
                //glDeleteTextures(1,&m_textureID);
            }
		};
	}
}
