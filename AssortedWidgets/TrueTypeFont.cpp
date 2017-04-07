#ifdef __APPLE__
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#else
#include <GLES2/gl2.h>
#endif
#include "TrueTypeFont.h"
#include <stdarg.h>
#define GLFONTSTASH_IMPLEMENTATION
#import "glfontstash.h"

namespace AssortedWidgets
{
	namespace Font
	{
        TrueTypeFont::TrueTypeFont(const char* _fontName,size_t _size):Font(_fontName,_size)
		{
            GLFONSparams params;
            params.useGLBackend = true; // if not set to true, you must provide your own gl backend
            m_stash = glfonsCreate(512, 512, FONS_ZERO_TOPLEFT | FONS_NORMALIZE_TEX_COORDS, params, nullptr);

            if ((m_fontNormal = fonsAddFont(m_stash, "Arial", _fontName)) == FONS_INVALID)
            {
               //printf("Can't load font\n");
                   // return 0;
            }
            m_size = _size;
		}

        Util::Size TrueTypeFont::getStringBoundingBox(const std::string &text)
		{
            float minx;
            float miny;
            float maxx;
            float maxy;

            bool isNew = false;
            fsuint textID = 0;
            fsuint buffer;

            glfonsBufferCreate(m_stash, &buffer);
            glfonsBindBuffer(m_stash, buffer);

            glfonsGenText(m_stash, 1, &textID);

            fonsSetSize(m_stash, m_size);
            glfonsRasterize(m_stash, textID, text.c_str());
            glfonsGetBBox(m_stash,  textID, &minx, &miny, &maxx, &maxy);
            glfonsBufferDelete(m_stash, buffer);

            return Util::Size(maxx-minx, maxy-miny);
        }

        void TrueTypeFont::drawString(int x, int y, const std::string &text)
		{   
            glfonsScreenSize(m_stash, 800, 600);
      /*      bool isNew = false;
            fsuint textID = findTextID(text, isNew);

            if (isNew)
            {
                fonsSetSize(m_stash, m_size);
                //glfonsRasterize(m_stash, textID, text.c_str());
            }
            fonsSetSize(m_stash, m_size);*/

            //glfonsSetColor(m_stash, (137 << 24) | (155 << 16) | (145 << 8) | 255 );
            //glfonsTransform(m_stash, textID, x, y, 0.0, 1.0);

           // glfonsUpdateBuffer(m_stash);
           // glfonsDraw(m_stash);
          //  fonsDrawText(m_stash, x,y,"The big ", NULL,0);
           // glfonsDraw(m_stash);

            float dx = 10, dy = 10;
           // unsigned int white = glfonsRGBA(255,255,255,255);
           // unsigned int brown = glfonsRGBA(192,128,0,128);
            fsuint textID = 0;
            fsuint buffer;
            fonsSetFont(m_stash, m_fontNormal);
           // fonsSetSize(m_stash, 124.0f);
           // fonsSetColor(m_stash, white);

            glfonsBufferCreate(m_stash, &buffer);
            glfonsBindBuffer(m_stash, buffer);

            glfonsGenText(m_stash, 1, &textID);
            glfonsSetColor(m_stash, m_color);

            fonsSetSize(m_stash, m_size);
            glfonsRasterize(m_stash, textID, text.c_str());
            glfonsTransform(m_stash, textID, x, y+9, 0.0, 1.0);
            glfonsUpdateBuffer(m_stash);
            glfonsDraw(m_stash);
            glfonsBufferDelete(m_stash, buffer);

          //  int a = len;
        }

        void TrueTypeFont::printf(int x,int y,const char *fmt, ...)
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
			
            drawString(x,y, text);

        }

        TrueTypeFont::~TrueTypeFont(void)
		{
            glfonsDelete(m_stash);
		}

        int TrueTypeFont::findTextID(const std::string &text, bool &isNew)
        {
            if (m_textIDs.find(text) != m_textIDs.end())
            {
                return m_textIDs[text];
                isNew = false;
            }
            else
            {
                fsuint textID;
                glfonsGenText(m_stash, 1, &textID);
                isNew = true;
                m_textIDs[text] = textID;
                return textID;
            }
        }

        void TrueTypeFont::setColor(int r, int g, int b)
        {
            m_color = glfonsRGBA(r,g,b,255);

        }

	}
}
