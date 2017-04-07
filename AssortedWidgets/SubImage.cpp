#include "SubImage.h"
#include <iostream>
#include "GraphicsBackend.h"

namespace AssortedWidgets
{
    namespace Theme
    {
        GLuint SubImage::m_vertShader = 0;
        GLuint SubImage::m_fragShader = 0;
        GLuint SubImage::m_shaderProgram = 0;
        GLint SubImage::m_screenSizeUniform = 0;
        GLint SubImage::m_textureUniform = 0;

        void SubImage::init(unsigned int width, unsigned int height)
        {
            const GLchar *vShaderStr =
               "attribute vec2 vPosition;   \n"
               "attribute vec2 vTexCoord;   \n"
               "varying vec2 v_TexCoordinate; \n"
               "uniform vec2 screenSize;    \n"
               "void main()                 \n"
               "{                           \n"
               "   v_TexCoordinate = vTexCoord; \n"
               "   gl_Position = vec4(vPosition.x * 2.0 / screenSize.x - 1.0, ( screenSize.y - vPosition.y) * 2.0 / screenSize.y - 1.0, 0.0, 1.0); \n"
               "}                           \n";

            const GLchar *fShaderStr =
               "precision mediump float;                   \n"
               "uniform sampler2D u_Texture;               \n"
               "varying vec2 v_TexCoordinate;              \n"
               "void main()                                \n"
               "{                                          \n"
               "  gl_FragColor = texture2D(u_Texture, v_TexCoordinate); \n"
               "}                                          \n";

            SubImage::m_vertShader = glCreateShader(GL_VERTEX_SHADER);
            glShaderSource(m_vertShader, 1, &vShaderStr, 0);

            glCompileShader(m_vertShader);

            GLint compiled;
            glGetShaderiv(m_vertShader, GL_COMPILE_STATUS, &compiled);



            SubImage::m_fragShader = glCreateShader(GL_FRAGMENT_SHADER);
            glShaderSource(m_fragShader, 1, &fShaderStr, 0);

            glCompileShader(m_fragShader);


            m_shaderProgram = glCreateProgram();

            glAttachShader(m_shaderProgram, m_vertShader);
            glAttachShader(m_shaderProgram, m_fragShader);

            glBindAttribLocation(m_shaderProgram, 0, "vPosition");
            glBindAttribLocation(m_shaderProgram, 1, "vTexCoord");

            glLinkProgram(m_shaderProgram);

            GLint length = 0;
            GLchar infoLog[1024];
            glGetProgramInfoLog(m_shaderProgram,  1024,  &length,  infoLog);

            std::cerr << infoLog;
            printf("fwefewfv %s\n", infoLog);

            glUseProgram(m_shaderProgram);
            m_screenSizeUniform = glGetUniformLocation(m_shaderProgram, "screenSize");
            m_textureUniform = glGetUniformLocation(m_shaderProgram, "u_Texture");

            printf("screensize");

        }

        void SubImage::paint(const float x1,const float y1,const float x2,const float y2) const
        {
            /*glColor3ub(255,255,255);
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
            glEnd();*/

            /*GLfloat vVertices[] = {x1,  y2,
                                   x1,  y1,
                                   x2,  y2,
                                   x2,  y1};
            GLfloat vTexCoords[] = {m_UpLeftX, m_BottomRightY,
                                   m_UpLeftX, m_UpLeftY,
                                   m_BottomRightX, m_BottomRightY,
                                   m_BottomRightX, m_UpLeftY};

            glUseProgram(m_shaderProgram);
            glUniform2f(m_screenSizeUniform, 800, 600);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, m_textureID);
            glUniform1i(m_textureUniform, 0);

            // Load the vertex data
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, vVertices);
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 0, vTexCoords);
            glEnableVertexAttribArray(1);
            glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
            glUseProgram(0);*/

            GraphicsBackend::getSingleton().drawTexturedQuad(x1, y1, x2, y2, m_UpLeftX, m_UpLeftY, m_BottomRightX, m_BottomRightY, m_textureID);
        }
    }
}
