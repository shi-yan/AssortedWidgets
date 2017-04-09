#include "GraphicsBackend.h"

namespace AssortedWidgets
{
    GraphicsBackend::GraphicsBackend()
        :m_texturedVertShader(0),
        m_texturedFragShader(0),
        m_texturedShaderProgram(0),
        m_texturedScreenSizeUniform(0),
        m_textureUniform(0)
    {

    }

    void GraphicsBackend::init(unsigned int width, unsigned int height)
    {
        m_width = width;
        m_height = height;

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
        #ifndef __APPLE__
           "precision mediump float;                   \n"
        #endif
           "uniform sampler2D u_Texture;               \n"
           "varying vec2 v_TexCoordinate;              \n"
           "void main()                                \n"
           "{                                          \n"
           "  gl_FragColor = texture2D(u_Texture, v_TexCoordinate); \n"
           "}                                          \n";

        m_texturedVertShader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(m_texturedVertShader, 1, &vShaderStr, 0);

        glCompileShader(m_texturedVertShader);

        m_texturedFragShader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(m_texturedFragShader, 1, &fShaderStr, 0);

        glCompileShader(m_texturedFragShader);


        m_texturedShaderProgram = glCreateProgram();

        glAttachShader(m_texturedShaderProgram, m_texturedVertShader);
        glAttachShader(m_texturedShaderProgram, m_texturedFragShader);

        glBindAttribLocation(m_texturedShaderProgram, 0, "vPosition");
        glBindAttribLocation(m_texturedShaderProgram, 1, "vTexCoord");

        glLinkProgram(m_texturedShaderProgram);

        glUseProgram(m_texturedShaderProgram);
        m_texturedScreenSizeUniform = glGetUniformLocation(m_texturedShaderProgram, "screenSize");
        m_textureUniform = glGetUniformLocation(m_texturedShaderProgram, "u_Texture");

        const GLchar *vSolidShaderStr =
           "attribute vec2 vPosition;   \n"
           "uniform vec2 screenSize;    \n"
           "void main()                 \n"
           "{                           \n"
           "   gl_Position = vec4(vPosition.x * 2.0 / screenSize.x - 1.0, ( screenSize.y - vPosition.y) * 2.0 / screenSize.y - 1.0, 0.0, 1.0); \n"
           "}                           \n";

        const GLchar *fSolidShaderStr =
        #ifndef __APPLE__
           "precision mediump float;                   \n"
        #endif
           "uniform vec4 color;                        \n"
           "void main()                                \n"
           "{                                          \n"
           "  gl_FragColor = color;                    \n"
           "}                                          \n";

        m_solidVertShader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(m_solidVertShader, 1, &vSolidShaderStr, 0);

        glCompileShader(m_solidVertShader);

        m_solidFragShader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(m_solidFragShader, 1, &fSolidShaderStr, 0);

        glCompileShader(m_solidFragShader);


        m_solidShaderProgram = glCreateProgram();

        glAttachShader(m_solidShaderProgram, m_solidVertShader);
        glAttachShader(m_solidShaderProgram, m_solidFragShader);

        glBindAttribLocation(m_solidShaderProgram, 0, "vPosition");

        glLinkProgram(m_solidShaderProgram);

        glUseProgram(m_solidShaderProgram);
        m_colorUniform = glGetUniformLocation(m_solidShaderProgram, "color");
        m_solidScreenSizeUniform = glGetUniformLocation(m_solidShaderProgram, "screenSize");


    }

    void GraphicsBackend::drawTexturedQuad(float x1, float y1, float x2, float y2,
                          float tx1, float ty1, float tx2, float ty2, GLuint textureID)
    {
        GLfloat vVertices[] = {x1,  y2,
                               x1,  y1,
                               x2,  y2,
                               x2,  y1};
        GLfloat vTexCoords[] = {tx1, ty2,
                               tx1, ty1,
                               tx2, ty2,
                               tx2, ty1};

        glUseProgram(m_texturedShaderProgram);
        glUniform2f(m_texturedScreenSizeUniform, m_width, m_height);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, textureID);
        glUniform1i(m_textureUniform, 0);

        // Load the vertex data
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, vVertices);
        glEnableVertexAttribArray(0);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 0, vTexCoords);
        glEnableVertexAttribArray(1);
        glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
        glUseProgram(0);
    }

    void GraphicsBackend::drawSolidQuad(float x1, float y1, float x2, float y2, float r, float g, float b, float a)
    {
        GLfloat vVertices[] = {x1,  y2,
                               x1,  y1,
                               x2,  y2,
                               x2,  y1};

        glUseProgram(m_solidShaderProgram);
        glUniform2f(m_solidScreenSizeUniform, m_width, m_height);
        glUniform4f(m_colorUniform, r/255.0, g/255.0, b/255.0, a);
        // Load the vertex data
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, vVertices);
        glEnableVertexAttribArray(0);
        glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
        glUseProgram(0);
    }

    void GraphicsBackend::drawLine(float x1, float y1, float x2, float y2, float r, float g, float b, float a )
    {
        GLfloat vVertices[] = {x1,  y1,
                               x2,  y2};

        glUseProgram(m_solidShaderProgram);
        glUniform2f(m_solidScreenSizeUniform, m_width, m_height);
        glUniform4f(m_colorUniform, r/255.0, g/255.0, b/255.0, a);
        // Load the vertex data
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, vVertices);
        glEnableVertexAttribArray(0);
        glDrawArrays(GL_LINE_STRIP, 0, 2);
        glUseProgram(0);
    }

    void GraphicsBackend::drawLineStrip(std::vector<float> &pointList, float r, float g, float b, float a )
    {
        glUseProgram(m_solidShaderProgram);
        glUniform2f(m_solidScreenSizeUniform, m_width, m_height);
        glUniform4f(m_colorUniform, r/255.0, g/255.0, b/255.0, a);
        // Load the vertex data
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, &pointList[0]);
        glEnableVertexAttribArray(0);
        glDrawArrays(GL_LINE_STRIP, 0, pointList.size()/2);
        glUseProgram(0);
    }
}
