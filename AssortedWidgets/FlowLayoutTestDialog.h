#pragma once
#include "Dialog.h"
#include "Button.h"
#include "Label.h"
#include "FlowLayout.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class FlowLayoutTestDialog:public Widgets::Dialog
		{
		private:
			Layout::FlowLayout *flowLayout;
			Widgets::Button *closeButton;
			Widgets::Label *TheLabel;
			Widgets::Label *quickLabel;
			Widgets::Label *brownLabel;
			Widgets::Label *foxLabel;
			Widgets::Label *jumpsLabel;
			Widgets::Label *overLabel;
			Widgets::Label *theLabel;
			Widgets::Label *lazyDogLabel;
		public:
			void onClose(const Event::MouseEvent &e);
			FlowLayoutTestDialog(void);
		public:
			~FlowLayoutTestDialog(void);
		};
	}
}