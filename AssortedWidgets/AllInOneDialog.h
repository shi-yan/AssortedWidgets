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
        class AllInOneDialog: public Widgets::Dialog
		{
		private:
            Widgets::Label *m_label;
            Widgets::Button *m_closeButton;
            Widgets::TextField *m_textField;
            Widgets::ScrollPanel *m_scrollPanel;
            Widgets::Label *m_labelInScroll;
            Widgets::CheckButton *m_check;
            Widgets::RadioButton *m_radio1;
            Widgets::RadioButton *m_radio2;
            Widgets::SlideBar *m_sliderH;
            Widgets::SlideBar *m_sliderV;
            Widgets::ProgressBar *m_progressH;
            Widgets::ProgressBar *m_progressV;
            Widgets::DropList *m_dropList1;
            Widgets::DropList *m_dropList2;
            Widgets::RadioGroup *m_radioGroup;
            Widgets::DropListItem *m_option1;
            Widgets::DropListItem *m_option2;
            Widgets::DropListItem *m_option3;
            Widgets::DropListItem *m_option4;
            Widgets::DropListItem *m_option5;
            Widgets::DropListItem *m_option6;
            Layout::GirdLayout *m_girdLayout;

		public:
			void onClose(const Event::MouseEvent &e);
			AllInOneDialog(void);

		public:
			~AllInOneDialog(void);
		};
	}
}
