#include "TypeAble.h"
#include "TypeActiveManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        TypeAble::TypeAble(void):m_active(false)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&TypeAble::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

        TypeAble::TypeAble(char *_text):m_text(_text),m_active(false)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&TypeAble::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

        TypeAble::TypeAble(std::string &_text):m_text(_text),m_active(false)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&TypeAble::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

		TypeAble::~TypeAble(void)
		{
		}

		void TypeAble::mousePressed(const Event::MouseEvent &e)
		{
			Manager::TypeActiveManager::getSingleton().setActive(this);
            m_active=true;
		}
	}
}
