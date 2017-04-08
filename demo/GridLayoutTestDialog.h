#pragma once
#include "Dialog.h"
#include "GridLayout.h"
#include "Label.h"
#include "Button.h"

namespace AssortedWidgets
{
	namespace Test
	{
        class GridLayoutTestDialog:public Widgets::Dialog
		{
		private:
            Layout::GridLayout *m_gridLayout;
            Widgets::Button *m_closeButton;
            Widgets::Label *m_label1;
            Widgets::Label *m_label2;
            Widgets::Label *m_label3;
            Widgets::Label *m_label4;
            Widgets::Label *m_label5;
            Widgets::Label *m_label6;
            Widgets::Label *m_label7;
            Widgets::Label *m_label8;
		public:
			void onClose(const Event::MouseEvent &e);
            GridLayoutTestDialog(void);
		public:
            ~GridLayoutTestDialog(void);
		};
	}
}
