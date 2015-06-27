#pragma once
#include "DragAble.h"
#include <string>
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Dialog;

        class DialogTittleBar: public DragAble
		{
		private:
			std::string text;
			Dialog *parent;
			unsigned int top;
			unsigned int bottom;
			unsigned int left;
			unsigned int right;
		public:
            const std::string& getText() const
			{
				return text;
            }
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
			void setDialog(Dialog *_parent)
			{
				parent=_parent;
			};
			DialogTittleBar(std::string &_text);
			DialogTittleBar(char *_text);
            Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getDialogTittleBarPreferedSize(this);
            }
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintDialogTittleBar(this);
			};
			void dragReleased(const Event::MouseEvent &e);
			void dragMoved(int offsetX,int offsetY);
		public:
			~DialogTittleBar(void);
		};
	}
}
