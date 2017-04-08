#include "TypeAble.h"
#include "TypeActiveManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        TypeAble::TypeAble(const std::string &_text):m_text(_text),m_active(false)
        {
            mousePressedHandlerList.push_back(MOUSE_DELEGATE(TypeAble::mousePressed));
		}

		TypeAble::~TypeAble(void)
		{
		}

		void TypeAble::mousePressed(const Event::MouseEvent &e)
		{
            (void) e;
			Manager::TypeActiveManager::getSingleton().setActive(this);
            m_active=true;
		}

        void TypeAble::onCharTyped(char character,int modifier)
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
	}
}
