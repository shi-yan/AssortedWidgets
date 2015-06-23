#pragma once
#include <vector>
#include "MouseEvent.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Menu;
		class MenuBar:public Component
		{
		private:
			std::vector<Menu*> menuList;
			std::vector<Component*> inList;
			int spacer;
			int leftSpacer;
			int topSpacer;
			int rightSpacer;
			int bottomSpacer;

			Menu *expandMenu;
			bool expand;

		private:
			MenuBar():leftSpacer(45),topSpacer(5),rightSpacer(45),bottomSpacer(5),expand(false),expandMenu(0)
			{
				MouseDelegate mMotion;
				mMotion.bind(this,&MenuBar::onMouseMove);
				mouseMovedHandlerList.push_back(mMotion);

				MouseDelegate mEnter;
				mEnter.bind(this,&MenuBar::onMouseEnter);
				mouseEnteredHandlerList.push_back(mEnter);

				MouseDelegate mExit;
				mExit.bind(this,&MenuBar::onMouseExit);
				mouseExitedHandlerList.push_back(mExit);

				MouseDelegate mPressed;
				mPressed.bind(this,&MenuBar::onMousePressed);
				mousePressedHandlerList.push_back(mPressed);

				MouseDelegate mReleased;
				mReleased.bind(this,&MenuBar::onMouseReleased);
				mouseReleasedHandlerList.push_back(mReleased);
			};

		public:

			void init(unsigned int width,unsigned int height=30,int _spacer=5)
			{
				spacer=_spacer;
				position.x=0;
				position.y=0;
				size.width=width;
				size.height=30;
			};

			static MenuBar& getSingleton()
			{
				static MenuBar obj;
				return obj;
			}

			Menu* getExpandMenu()
			{
				return expandMenu;
			};

			bool isExpand()
			{
				return expand;
			};

			void setExpand(Menu *_expandMenu);

			void setShrink();

			void addMenu(Menu *menu);

			void paint();

			Util::Size getPreferedSize()
			{
				return size;
			};

			void onMouseEnter(const Event::MouseEvent &e);

			void onMouseExit(const Event::MouseEvent &e);

			void onMousePressed(const Event::MouseEvent &e);

			void onMouseReleased(const Event::MouseEvent &e);

			void onMouseMove(const Event::MouseEvent &e);

			void updateLayout();


		public:
			~MenuBar(void);
		};
	}
}