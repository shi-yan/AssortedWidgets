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
        class PanelTestDialog: public Widgets::Dialog
		{
		private:
            Widgets::Button *m_closeButton;
            Widgets::Label *m_label;
            Widgets::ScrollPanel *m_panel;
            Layout::GirdLayout *m_girdLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			PanelTestDialog(void);
		public:
			~PanelTestDialog(void);
		};
	}
}
