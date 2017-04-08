#pragma once
#include "Dialog.h"
#include "BorderLayout.h"
#include "Label.h"
#include "Button.h"

namespace AssortedWidgets
{
	namespace Test
	{
        class BorderLayoutTestDialog: public Widgets::Dialog
		{
		private:
            Widgets::Button *m_closeButton;
            Widgets::Label *m_northLabel;
            Widgets::Label *m_southLabel;
            Widgets::Label *m_westLabel;
            Widgets::Label *m_eastLabel;
            Widgets::Label *m_centerLabel;
            Layout::BorderLayout *m_borderLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			BorderLayoutTestDialog(void);
		public:
			~BorderLayoutTestDialog(void);
		};
	}
}
