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

		class Element: virtual public Component
		{
		protected:
            Container *m_parent;
            int m_horizontalStyle;
            int m_verticalStyle;
		public:
			enum Style
			{
				Any,
				Fit,
				Stretch
            };

            Element()
                :m_parent(NULL),
                  m_horizontalStyle(0),
                  m_verticalStyle(0)
            {}
           	
			void setHorizontalStyle(int _horizontalStyle)
			{
                m_horizontalStyle=_horizontalStyle;
            }

			void setVerticalStyle(int _verticalStyle)
			{
                m_verticalStyle=_verticalStyle;
            }

            int getHorizontalStyle() const
			{
                return m_horizontalStyle;
            }

            int getVerticalStyle() const
			{
                return m_verticalStyle;
            }

			void setParent(Container *_parent)
			{
                m_parent=_parent;
            }

			Container& getParent()
			{
                return *m_parent;
            }

            virtual ~Element()
            {}
		};

		class Container:virtual public Component
		{
		protected:
			std::vector<Element*> childList;
			Manager::SelectionManager selectionManager;
			Layout::Layout *layout;
		public:
            Container(void)
                :layout(0)
            {}
			void add(Element *child)
			{
				childList.push_back(child);
            }
			void setLayout(Layout::Layout *_layout)
			{
				if(layout)
				{
					delete layout;
				}
				layout=_layout;
            }
			void remove(Element *child)
			{
				std::vector<Element*>::iterator iter = std::find(childList.begin(), childList.end(),child);
				if(iter != childList.end())
				{
					(*iter)->setParent(0);
					delete (*iter);
					childList.erase(iter);
				}
            }
			virtual void paintChild() = 0;
		public:
			virtual ~Container(void)
			{
			/*	for(std::vector<Element*>::iterator iter=childList.begin();iter<childList.end();++iter)
				{
					delete (*iter);
				}*/
				childList.clear();
            }
		};
	}
}
