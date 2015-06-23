#pragma once
#include "Dialog.h"
#include "BorderLayout.h"
#include "Label.h"
#include "Button.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class BorderLayoutTestDialog:public Widgets::Dialog
		{
		private:
			Widgets::Button *closeButton;
			Widgets::Label *northLabel;
			Widgets::Label *southLabel;
			Widgets::Label *westLabel;
			Widgets::Label *eastLabel;
			Widgets::Label *centerLabel;
			Layout::BorderLayout *borderLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			BorderLayoutTestDialog(void);
		public:
			~BorderLayoutTestDialog(void);
		};
	}
}