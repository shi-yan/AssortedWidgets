#pragma once
#include "Dialog.h"
#include "GirdLayout.h"
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
			Layout::GirdLayout *girdLayout;
			Widgets::Button *closeButton;
			Widgets::CheckButton *checkButton1;
			Widgets::CheckButton *checkButton2;
			Widgets::CheckButton *checkButton3;
			Widgets::RadioButton *radioButton1;
			Widgets::RadioButton *radioButton2;
			Widgets::RadioButton *radioButton3;
			Widgets::RadioGroup *radioGroup;
			Widgets::Spacer *spacer;
		public:
			CheckNRadioTestDialog(void);
			void onClose(const Event::MouseEvent &e);
		public:
			~CheckNRadioTestDialog(void);
		};
	}
}