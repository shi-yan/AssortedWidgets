#pragma once
#include "ContainerElement.h"
#include "KeyEvent.h"
#include <ctype.h>
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
        class TypeAble: public Element
		{
		private:
            std::string m_text;
            bool m_active;
		public:
			TypeAble(char *_text);
			TypeAble(std::string &_text);
			TypeAble(void);
			bool isActive()
			{
                return m_active;
            }
            const std::string& getText() const
			{
                return m_text;
			}
			void setActive(bool _active)
			{
                m_active=_active;
            }
			void mousePressed(const Event::MouseEvent &e);
			void onCharTyped(char character,int modifier)
			{
                if(character==8 && m_text.length())
				{
                    m_text.erase(m_text.length()-1);
				}
				else
				{
					if((modifier & Event::KeyEvent::MOD_LSHIFT) ||(modifier & Event::KeyEvent::MOD_RSHIFT) ||(modifier & Event::KeyEvent::MOD_CAPS))
					{
                        m_text+=toupper(character);
					}
					else
					{
                        m_text+=character;
					}
				}
            }
		public:
			~TypeAble(void);
		};
	}
}
