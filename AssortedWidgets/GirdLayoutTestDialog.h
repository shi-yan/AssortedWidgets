#pragma once
#include "Dialog.h"
#include "GirdLayout.h"
#include "Label.h"
#include "Button.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class GirdLayoutTestDialog:public Widgets::Dialog
		{
		private:
			Layout::GirdLayout *girdLayout;
			Widgets::Button *closeButton;
			Widgets::Label *label1;
			Widgets::Label *label2;
			Widgets::Label *label3;
			Widgets::Label *label4;
			Widgets::Label *label5;
			Widgets::Label *label6;
			Widgets::Label *label7;
			Widgets::Label *label8;
		public:
			void onClose(const Event::MouseEvent &e);
			GirdLayoutTestDialog(void);
		public:
			~GirdLayoutTestDialog(void);
		};
	}
}