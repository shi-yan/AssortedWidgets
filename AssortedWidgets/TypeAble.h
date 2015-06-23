#pragma once
#include "ContainerElement.h"
#include "KeyEvent.h"
#include <ctype.h>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class TypeAble:public Element
		{
		private:
			std::string text;
			bool active;
		public:
			TypeAble(char *_text);
			TypeAble(std::string &_text);
			TypeAble(void);
			bool isActive()
			{
				return active;
			};
			std::string getText()
			{
				return text;
			}
			void setActive(bool _active)
			{
				active=_active;
			};
			void mousePressed(const Event::MouseEvent &e);
			void onCharTyped(char character,int modifier)
			{
				if(character==8 && text.length())
				{
					text.erase(text.length()-1);
				}
				else
				{
					if((modifier & Event::KeyEvent::MOD_LSHIFT) ||(modifier & Event::KeyEvent::MOD_RSHIFT) ||(modifier & Event::KeyEvent::MOD_CAPS))
					{
						text+=toupper(character);
					}
					else
					{
						text+=character;
					}
				}
			};
		public:
			~TypeAble(void);
		};
	}
}