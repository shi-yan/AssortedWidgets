#pragma once
#include "Component.h"
#include "MenuList.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
        class MenuItem: public Component
		{
		protected:
            MenuList *m_parentMenuList;
		public:
			MenuItem(void);
	
			void setMenuList(MenuList *_menuList)
			{
                m_parentMenuList=_menuList;
			}
		//	void paint(){};
		public:
			~MenuItem(void);
		};
	}
}
