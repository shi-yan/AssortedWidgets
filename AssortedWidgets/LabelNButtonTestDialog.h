#pragma once
#include "Dialog.h"
#include "GirdLayout.h"
#include "Label.h"
#include "Button.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class LabelNButtonTestDialog:public Widgets::Dialog
		{
		private:
			Layout::GirdLayout *girdLayout;
			Widgets::Button *testButton;
			Widgets::Button *closeButton;
			Widgets::Label *testLabel;

		public:
			LabelNButtonTestDialog(void);
			void onClose(const Event::MouseEvent &e);
		public:
			~LabelNButtonTestDialog(void);
		};
	}
}