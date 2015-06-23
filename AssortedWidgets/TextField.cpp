#include "TextField.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		TextField::TextField(unsigned int _length):TypeAble(),length(_length),top(4)
		{
			size.width=length+12;
			size.height=20;
		}
			
		TextField::TextField(unsigned int _length,char *_text):TypeAble(_text),length(_length),top(4)
		{
			size.width=length+12;
			size.height=20;
		}

		TextField::TextField(unsigned int _length,std::string &_text):TypeAble(_text),length(_length),top(4)
		{
			size.width=length+12;
			size.height=20;
		}

		TextField::~TextField(void)
		{
		}
	}
}