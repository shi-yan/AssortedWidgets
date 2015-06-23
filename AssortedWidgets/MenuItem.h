#pragma once
#include "Component.h"
#include "MenuList.h"
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class MenuItem:public Component
		{
		protected:
			MenuList *parentMenuList;
		public:
			MenuItem(void);
	
			void setMenuList(MenuList *_menuList)
			{
				parentMenuList=_menuList;
			}
		//	void paint(){};
		public:
			~MenuItem(void);
		};
	}
}