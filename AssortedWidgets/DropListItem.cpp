#include "DropListItem.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		DropListItem::DropListItem(char *_text):text(_text)
		{
            m_size=getPreferedSize();
		}

		DropListItem::DropListItem(std::string &_text):text(_text)
		{
            m_size=getPreferedSize();
		}

		DropListItem::~DropListItem(void)
		{
		}
	}
}
