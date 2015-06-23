#pragma once
#include "DragAble.h"
#include "SlideBar.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class SlideBarSlider:public DragAble
		{
		private:
			SlideBar *parent;
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
			void setSlideBar(SlideBar *_parent)
			{
				parent=_parent;
			};
			SlideBarSlider(int _type);
			Util::Size getPreferedSize()
			{
				return size;
			};

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintSlideBarSlider(this);
			};
						
			void dragReleased(const Event::MouseEvent &e)
			{};

			void dragMoved(int offsetX,int offsetY)
			{
				if(type==Horizontal)
				{
					position.x+=offsetX;
					if(position.x<2)
					{
						position.x=2;
					}
					else if(position.x>static_cast<int>(parent->size.width-2-size.width))
					{
						position.x=parent->size.width-2-size.width;
					}
					parent->setPercent(std::min<float>(1.0f,static_cast<float>(position.x-2)/static_cast<float>(parent->size.width-4-size.width)));
				}
				else if(type==Vertical)
				{
					position.y+=offsetY;
					if(position.y<2)
					{
						position.y=2;
					}
					else if(position.y>static_cast<int>(parent->size.height-2-size.height))
					{
						position.y=parent->size.height-2-size.height;
					}
					parent->setPercent(std::min<float>(1.0f,static_cast<float>(position.y-2)/static_cast<float>(parent->size.height-4-size.height)));
				}
//				parent->onValueChanged();
			};

		public:
			~SlideBarSlider(void);
		};
	}
}