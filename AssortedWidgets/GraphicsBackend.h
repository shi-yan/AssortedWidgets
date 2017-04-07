#ifndef GRAPHICSBACKEND_H
#define GRAPHICSBACKEND_H

#include <GLES2/gl2.h>
#include <vector>

namespace AssortedWidgets
{
    class GraphicsBackend
    {
    private:
        GraphicsBackend();

        unsigned int m_width;
        unsigned int m_height;


        GLuint m_texturedVertShader;
        GLuint m_texturedFragShader;
        GLuint m_texturedShaderProgram;
        GLint m_texturedScreenSizeUniform;
        GLint m_textureUniform;

        GLuint m_solidVertShader;
        GLuint m_solidFragShader;
        GLuint m_solidShaderProgram;
        GLuint m_solidScreenSizeUniform;
        GLint m_colorUniform;

    public:
        static GraphicsBackend &getSingleton()
        {
            static GraphicsBackend obj;
            return obj;
        }

        void init(unsigned int width, unsigned int height);

        void drawTexturedQuad(float x1, float y1, float x2, float y2,
                              float tx1, float ty1, float tx2, float ty2, GLuint textureID);

        void drawSolidQuad(float x1, float y1, float x2, float y2, float r, float g, float b, float a = 1.0);
        void drawLine(float x1, float y1, float x2, float y2, float r, float g, float b, float a = 1.0);

        void drawLineStrip(std::vector<float> &pointList, float r, float g, float b, float a = 1.0);
    };
}
#endif // GRAPHICSBACKEND_H
