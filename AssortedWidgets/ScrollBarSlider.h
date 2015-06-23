#pragma once
#include "DragAble.h"
#include "ThemeEngine.h"
#include "ScrollBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ScrollBarSlider:public DragAble
		{
		private:
			ScrollBar *parent;
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};
		private:
			int type;
		public:
			int getType()
			{
				return type;
			}
			void setScrollBar(ScrollBar *_parent)
			{
				parent=_parent;
			};
			ScrollBarSlider(int _type);
			Util::Size getPreferedSize()
			{
				return size;
			};

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintScrollBarSlider(this);
			};

			void dragReleased(const Event::MouseEvent &e)
			{};

			void dragMoved(int offsetX,int offsetY)
			{
				if(type==Horizontal)
				{
					position.x+=offsetX;
					if(position.x<17)
					{
						position.x=17;
					}
					else if(position.x>static_cast<int>(parent->size.width-17-size.width))
					{
						position.x=parent->size.width-17-size.width;
					}
					parent->setValue(static_cast<float>(position.x-17)/static_cast<float>(parent->size.width-34-size.width));
				}
				else if(type==Vertical)
				{
					position.y+=offsetY;
					if(position.y<17)
					{
						position.y=17;
					}
					else if(position.y>static_cast<int>(parent->size.height-17-size.height))
					{
						position.y=parent->size.height-17-size.height;
					}
					parent->setValue(static_cast<float>(position.y-17)/static_cast<float>(parent->size.height-34-size.height));
				}
				parent->onValueChanged();
			};

		public:
			~ScrollBarSlider(void);
		};
	}
}