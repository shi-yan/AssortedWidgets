#pragma once
#include "Dialog.h"
#include "GridLayout.h"
#include "Button.h"
#include "CheckButton.h"
#include "RadioButton.h"
#include "RadioGroup.h"
#include "Spacer.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class CheckNRadioTestDialog:public Widgets::Dialog
		{
		private:
            Layout::GridLayout *m_gridLayout;
            Widgets::Button *m_closeButton;
            Widgets::CheckButton *m_checkButton1;
            Widgets::CheckButton *m_checkButton2;
            Widgets::CheckButton *m_checkButton3;
            Widgets::RadioButton *m_radioButton1;
            Widgets::RadioButton *m_radioButton2;
            Widgets::RadioButton *m_radioButton3;
            Widgets::RadioGroup *m_radioGroup;
            Widgets::Spacer *m_spacer;
		public:
			CheckNRadioTestDialog(void);
			void onClose(const Event::MouseEvent &e);
		public:
			~CheckNRadioTestDialog(void);
		};
	}
}
