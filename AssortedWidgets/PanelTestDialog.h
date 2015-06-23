#pragma once
#include "Dialog.h"
#include "GirdLayout.h"
#include "Button.h"
#include "Label.h"
#include "ScrollPanel.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class PanelTestDialog:public Widgets::Dialog
		{
		private:
			Widgets::Button *closeButton;
			Widgets::Label *label;
			Widgets::ScrollPanel *panel;
			Layout::GirdLayout *girdLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			PanelTestDialog(void);
		public:
			~PanelTestDialog(void);
		};
	}
}