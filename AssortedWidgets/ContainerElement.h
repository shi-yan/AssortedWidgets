#pragma once

#include <vector>
#include "Component.h"
#include "SelectionManager.h"
#include "Layout.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Container;
		//class Element;

		class Element: virtual public Component
		{
		protected:
			Container *parent;
			//检查一下拼写
			int horizontalStyle;
			int verticalStyle;
		public:
			enum Style
			{
				Any,
				Fit,
				//检查一下拼写
				Stretch
			};
           	
			//检查一下拼写
			void setHorizontalStyle(int _horizontalStyle)
			{
				horizontalStyle=_horizontalStyle;
			};

			void setVerticalStyle(int _verticalStyle)
			{
				verticalStyle=_verticalStyle;
			};

			//检查一下拼写
			int getHorizontalStyle()
			{
				return horizontalStyle;
			};

			int getVerticalStyle()
			{
				return verticalStyle;
			};

			void setParent(Container *_parent)
			{
				parent=_parent;
			};

			Container& getParent()
			{
				return *parent;
			};
		};

		class Container:virtual public Component
		{
		protected:
			std::vector<Element*> childList;
			Manager::SelectionManager selectionManager;
			Layout::Layout *layout;
		public:
			Container(void):layout(0)
			{};
			void add(Element *child)
			{
				childList.push_back(child);
			};
			void setLayout(Layout::Layout *_layout)
			{
				if(layout)
				{
					delete layout;
				}
				layout=_layout;
			};
			void remove(Element *child)
			{
				std::vector<Element*>::iterator iter = std::find(childList.begin(), childList.end(),child);
				if(iter != childList.end())
				{
					(*iter)->setParent(0);
					delete (*iter);
					childList.erase(iter);
				}
			};
			virtual void paintChild() = 0;
		public:
			virtual ~Container(void)
			{
			/*	for(std::vector<Element*>::iterator iter=childList.begin();iter<childList.end();++iter)
				{
					delete (*iter);
				}*/
				childList.clear();
			};
		};
	}
}
