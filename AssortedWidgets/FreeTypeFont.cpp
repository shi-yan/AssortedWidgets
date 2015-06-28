#include <GL/gl.h>
#include <GL/glu.h>
#include "FreeTypeFont.h"

namespace AssortedWidgets
{
	namespace Font
	{
		FreeTypeFont::FreeTypeFont(char* _fontName,size_t _size):Font(_fontName,_size)
		{
            m_ftfont = new OGLFT::TranslucentTexture(_fontName,static_cast<float>(_size));
			for(int i = 0; i < 255; ++i)
			{
                OGLFT::BBox bbox = m_ftfont->measure(static_cast<unsigned char>(i));
                m_width[i] = static_cast<int>(bbox.advance_.dx_);
                m_height[i] = static_cast<int>(bbox.advance_.dy_ + bbox.y_max_);
                m_fontCache.push_back(Util::Size(m_width[i], m_height[i]));
			}
		}

		Util::Size FreeTypeFont::getStringBoundingBox(const std::string &text) const
		{
			Util::Size result(0,0);

			for(size_t i = 0; i < text.length(); ++i)
			{
				unsigned char c=text[i];
                result.m_width+=m_width[static_cast<int>(c)];
                result.m_height=std::max(result.m_height,m_height[static_cast<int>(c)]);
			}
			return result;
        }

		void FreeTypeFont::drawString(int x, int y, const std::string &text) const
		{   
			glEnable(GL_TEXTURE_2D);
		    glEnable( GL_BLEND );
			glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
			glPushMatrix();
            glTranslatef(static_cast<GLfloat>(x),static_cast<GLfloat>(y + getStringBoundingBox(text).m_height) ,0);
			glScalef(1,-1,1);
            m_ftfont->draw(text.c_str());
			glPopMatrix();
			glDisable(GL_TEXTURE_2D);
        }

		void FreeTypeFont::printf(int x,int y,const char *fmt, ...) const
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
			glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
			glPushMatrix();
            glTranslatef(static_cast<GLfloat>(x),static_cast<GLfloat>(y + getStringBoundingBox(text).m_height) ,0);
			glScalef(1,-1,1);
            m_ftfont->draw(text);
			glPopMatrix();
			glDisable(GL_TEXTURE_2D);
        }

		FreeTypeFont::~FreeTypeFont(void)
		{
            delete m_ftfont;
		}
	}
}
