#pragma once
#include "ContainerElement.h"
#include "DropListButton.h"
#include "DropListItem.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class DropList:public Element
		{
		private:
            DropListButton m_button;
            std::vector<DropListItem*> m_itemList;
            DropListItem *m_selectedItem;
            unsigned int m_spacer;
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            bool m_dropped;

		public:
            bool isDropped() const
			{
                return m_dropped;
            }

			void shrinkBack()
			{
                m_dropped=false;
            }

			DropList(void);
			void setSpacer(unsigned int _spacer)
			{
                m_spacer=_spacer;
            }

			void setTop(unsigned int _top)
			{
                m_top=_top;
            }

			void setBottom(unsigned int _bottom)
			{
                m_bottom=_bottom;
            }

			void setLeft(unsigned int _left)
			{
                m_left=_left;
            }

			void setRight(unsigned int _right)
			{
                m_right=_right;
            }

            unsigned int getTop() const
			{
                return m_top;
            }

            unsigned int getBottom() const
			{
                return m_bottom;
            }

            unsigned int getLeft() const
			{
                return m_left;
            }

            unsigned int getRight() const
			{
                return m_right;
            }

            unsigned int getSpacer() const
			{
                return m_spacer;
            }

			std::vector<DropListItem*> &getItemList()
			{
                return m_itemList;
            }

			DropListItem* getSelectedItem()
			{
                return m_selectedItem;
            }

			void add(DropListItem *item)
			{
                m_itemList.push_back(item);
                m_size=getPreferedSize();
            }

			void setSelection(size_t index)
			{
                m_selectedItem=m_itemList[index];
            }

			void setSelection(DropListItem *selected)
			{
                m_selectedItem=selected;
			}

			Util::Size getPreferedSize()
			{
				unsigned miniSize=0;
				std::vector<DropListItem*>::iterator iter;
                for(iter=m_itemList.begin(); iter<m_itemList.end();++iter)
				{
                    miniSize=std::max<unsigned int>((*iter)->getPreferedSize().m_width,miniSize);
				}
				return Util::Size(miniSize+23,20);
            }
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void mouseReleased(const Event::MouseEvent &e);

			void mouseEntered(const Event::MouseEvent &e);
			void mouseExited(const Event::MouseEvent &e);
			void mouseMoved(const Event::MouseEvent &e);

			void onDropReleased(const Event::MouseEvent &e);
			void pack();
		public:
			~DropList(void);
		};
	}
}
