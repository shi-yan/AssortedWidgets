#include "SubImage.h"
#include <iostream>
#include "GraphicsBackend.h"

namespace AssortedWidgets
{
    namespace Theme
    {
        void SubImage::paint(const float x1,const float y1,const float x2,const float y2) const
        {
            GraphicsBackend::getSingleton().drawTexturedQuad(x1, y1, x2, y2, m_UpLeftX, m_UpLeftY, m_BottomRightX, m_BottomRightY, m_textureID);
        }
    }
}
