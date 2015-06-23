#pragma once
#include "Dialog.h"
#include "GirdLayout.h"

#include "Label.h"
#include "Button.h"
#include "TextField.h"
#include "ScrollPanel.h"
#include "CheckButton.h"
#include "RadioButton.h"
#include "SlideBar.h"
#include "ProgressBar.h"
#include "DropList.h"

#include "DropListItem.h"
#include "RadioGroup.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class AllInOneDialog:public Widgets::Dialog
		{
		private:
			Widgets::Label *label;
			Widgets::Button *closeButton;
			Widgets::TextField *textField;
			Widgets::ScrollPanel *scrollPanel;
			Widgets::Label *labelInScroll;
			Widgets::CheckButton *check;
			Widgets::RadioButton *radio1;
			Widgets::RadioButton *radio2;
			Widgets::SlideBar *sliderH;
			Widgets::SlideBar *sliderV;
			Widgets::ProgressBar *progressH;
			Widgets::ProgressBar *progressV;
			Widgets::DropList *dropList1;
			Widgets::DropList *dropList2;

			Widgets::RadioGroup *radioGroup;
			Widgets::DropListItem *option1;
			Widgets::DropListItem *option2;
			Widgets::DropListItem *option3;

			Widgets::DropListItem *option4;
			Widgets::DropListItem *option5;
			Widgets::DropListItem *option6;
			Layout::GirdLayout *girdLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			AllInOneDialog(void);
		public:
			~AllInOneDialog(void);
		};
	}
}