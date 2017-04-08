#pragma once
#include "Dialog.h"
#include "GridLayout.h"
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
            Widgets::Button *m_closeButton;
            Layout::GridLayout *m_gridLayout;
            Widgets::CheckButton *m_dragAble;
            Widgets::CheckButton *m_sizeAble;
            Widgets::Label *m_label;
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
