#include "TypeAble.h"
#include "TypeActiveManager.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		TypeAble::TypeAble(void):active(false)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&TypeAble::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

		TypeAble::TypeAble(char *_text):text(_text),active(false)
		{
			MouseDelegate mPressed;
			mPressed.bind(this,&TypeAble::mousePressed);
			mousePressedHandlerList.push_back(mPressed);
		}

		TypeAble::TypeAble(std::string &_text):text(_text),active(false)
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
			active=true;
		}
	}
}