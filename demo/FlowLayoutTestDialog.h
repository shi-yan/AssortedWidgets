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
            Layout::FlowLayout *m_flowLayout;
            Widgets::Button *m_closeButton;
            Widgets::Label *m_TheLabel;
            Widgets::Label *m_quickLabel;
            Widgets::Label *m_brownLabel;
            Widgets::Label *m_foxLabel;
            Widgets::Label *m_jumpsLabel;
            Widgets::Label *m_overLabel;
            Widgets::Label *m_aLabel;
            Widgets::Label *m_lazyDogLabel;
		public:
			void onClose(const Event::MouseEvent &e);
			FlowLayoutTestDialog(void);
		public:
			~FlowLayoutTestDialog(void);
		};
	}
}
