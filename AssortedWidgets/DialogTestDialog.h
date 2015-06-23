#pragma once
#include "Dialog.h"
#include "GirdLayout.h"
#include "Button.h"
#include "CheckButton.h"
#include "Label.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class DialogTestDialog:public Widgets::Dialog
		{
		private:
			Widgets::Button *closeButton;
			Layout::GirdLayout *girdLayout;
			Widgets::CheckButton *dragAble;
			Widgets::CheckButton *sizeAble;
			Widgets::Label *label;
		public:
			void onClose(const Event::MouseEvent &e);
			void onDrag(const Event::MouseEvent &e);
			void onSize(const Event::MouseEvent &e);
			DialogTestDialog(void);
		public:
			~DialogTestDialog(void);
		};
	}
}