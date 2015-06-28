#include "TextField.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        TextField::TextField(unsigned int _length)
            :TypeAble(),
              m_length(_length),
              m_top(4)
		{
            m_size.width=m_length+12;
            m_size.height=20;
		}
			
        TextField::TextField(unsigned int _length,char *_text)
            :TypeAble(_text),
              m_length(_length),
              m_top(4)
		{
            m_size.width=m_length+12;
            m_size.height=20;
		}

        TextField::TextField(unsigned int _length,std::string &_text)
            :TypeAble(_text),
              m_length(_length),
              m_top(4)
		{
            m_size.width=m_length+12;
            m_size.height=20;
		}

		TextField::~TextField(void)
		{
		}
	}
}
