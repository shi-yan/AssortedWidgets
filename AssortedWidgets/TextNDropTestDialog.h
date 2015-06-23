#pragma once
#include "Dialog.h"
#include "TextField.h"
#include "DropList.h"
#include "GirdLayout.h"
#include "Button.h"
#include "DropListItem.h"
#include "Label.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class TextNDropTestDialog:public Widgets::Dialog
		{
		private:
			Widgets::Button *closeButton;
			Widgets::TextField *textField;
			Widgets::DropList *dropList;
			Widgets::DropListItem *option1;
			Widgets::DropListItem *option2;
			Widgets::DropListItem *option3;
			Layout::GirdLayout *girdLayout;
			Widgets::Label *optionLabel;
			Widgets::Label *textLabel;
		public:
			void onClose(const Event::MouseEvent &e);
			TextNDropTestDialog(void);
		public:
			~TextNDropTestDialog(void);
		};
	}
}