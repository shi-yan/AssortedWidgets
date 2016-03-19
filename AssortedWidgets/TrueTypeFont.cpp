#ifdef __APPLE__
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GL/gl.h>
#include <GL/glu.h>
#endif
#include "TrueTypeFont.h"
#include <stdarg.h>
namespace AssortedWidgets
{
	namespace Font
	{
        TrueTypeFont::TrueTypeFont(char* _fontName,size_t _size):Font(_fontName,_size)
		{
            m_stash = sth_create(512, 512);
            m_font = sth_add_font(m_stash, _fontName);
            m_size = _size;
		}

        Util::Size TrueTypeFont::getStringBoundingBox(const std::string &text) const
		{
        /*	Util::Size result(0,0);

			for(size_t i = 0; i < text.length(); ++i)
			{
				unsigned char c=text[i];
                result.m_width+=m_width[static_cast<int>(c)];
                result.m_height=std::max(result.m_height,m_height[static_cast<int>(c)]);
			}
            return result;*/
            float minx;
            float miny;
            float maxx;
            float maxy;
            sth_dim_text(m_stash,
                              m_font, m_size,
                              text.c_str(),
                              &minx, &miny, &maxx, &maxy);
            return Util::Size(maxx-minx, maxy-miny);
        }

        void TrueTypeFont::drawString(int x, int y, const std::string &text) const
		{   
            glEnable(GL_TEXTURE_2D);
            glEnable( GL_BLEND );
            glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
          //  glPushMatrix();
               // glTranslatef(static_cast<GLfloat>(x),static_cast<GLfloat>(y + getStringBoundingBox(text).m_height) ,0);
               // glScalef(1,-1,1);
            sth_begin_draw(m_stash);
            float x2;
            sth_draw_text(m_stash, m_font, m_size, x, y, text.c_str(), &x2);

            sth_end_draw(m_stash);
          //  glPopMatrix();

                        glDisable(GL_TEXTURE_2D);
        }

        void TrueTypeFont::printf(int x,int y,const char *fmt, ...) const
		{
			char text[256];
			va_list	ap;
			if (fmt == NULL)
				return;
			else
			{
				va_start(ap, fmt);
				vsprintf(text, fmt, ap);
				va_end(ap);
			}
			

            glEnable(GL_TEXTURE_2D);
            glEnable( GL_BLEND );
            glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
         //   glPushMatrix();
               // glTranslatef(static_cast<GLfloat>(x),static_cast<GLfloat>(y + getStringBoundingBox(text).m_height) ,0);
            //    glScalef(1,-1,1);
            sth_begin_draw(m_stash);
            float x2;
            sth_draw_text(m_stash, m_font, m_size, x, y, text, &x2);
            sth_end_draw(m_stash);
           // glPopMatrix();
            glDisable(GL_TEXTURE_2D);

        }

        TrueTypeFont::~TrueTypeFont(void)
		{
            sth_delete(m_stash);
		}
	}
}
