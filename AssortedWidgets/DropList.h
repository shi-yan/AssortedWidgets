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
			DropListButton button;
			std::vector<DropListItem*> itemList;
			DropListItem *selectedItem;
			unsigned int spacer;
			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;
			bool dropped;
		public:
			bool isDropped()
			{
				return dropped;
			};

			void shrinkBack()
			{
				dropped=false;
			};

			DropList(void);
			void setSpacer(unsigned int _spacer)
			{
				spacer=_spacer;
			};

			void setTop(unsigned int _top)
			{
				top=_top;
			};

			void setBottom(unsigned int _bottom)
			{
				bottom=_bottom;
			};

			void setLeft(unsigned int _left)
			{
				left=_left;
			};

			void setRight(unsigned int _right)
			{
				right=_right;
			};

			unsigned int getTop()
			{
				return top;
			};

			unsigned int getBottom()
			{
				return bottom;
			};

			unsigned int getLeft()
			{
				return left;
			};

			unsigned int getRight()
			{
				return right;
			};


			unsigned int getSpacer()
			{
				return spacer;
			};

			std::vector<DropListItem*> &getItemList()
			{
				return itemList;
			};
			DropListItem* getSelectedItem()
			{
				return selectedItem;
			};
			void add(DropListItem *item)
			{
				itemList.push_back(item);
				size=getPreferedSize();
			};
			void setSelection(size_t index)
			{
				selectedItem=itemList[index];
			};
			void setSelection(DropListItem *selected)
			{
				selectedItem=selected;
			}
			Util::Size getPreferedSize()
			{
				unsigned miniSize=0;
				std::vector<DropListItem*>::iterator iter;
				for(iter=itemList.begin();iter<itemList.end();++iter)
				{
					miniSize=std::max<unsigned int>((*iter)->getPreferedSize().width,miniSize);
				}
				return Util::Size(miniSize+23,20);
			};
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