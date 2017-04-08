#include "DropListItem.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        DropListItem::DropListItem(const std::string &_text)
            :m_text(_text)
		{
            m_size=getPreferedSize();
		}

		DropListItem::~DropListItem(void)
		{
		}
	}
}
