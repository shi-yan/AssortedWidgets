#pragma once
#include "Dialog.h"
#include "BorderLayout.h"
#include "Panel.h"
#include "FlowLayout.h"
#include "GridLayout.h"
#include "Button.h"
#include "Label.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class MultipleLayoutTestDialog:public Widgets::Dialog
		{
		private:
            Layout::GridLayout *m_gridLayout;
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

            Widgets::Label *m_northLabel;
            Widgets::Label *m_southLabel;
            Widgets::Label *m_westLabel;
            Widgets::Label *m_eastLabel;
            Widgets::Label *m_centerLabel;
            Layout::BorderLayout *m_borderLayout;

            Widgets::Panel *m_flowPanel;
            Widgets::Panel *m_borderPanel;





		public:
			void onClose(const Event::MouseEvent &e);
			MultipleLayoutTestDialog(void);
		public:
			~MultipleLayoutTestDialog(void);
		};
	}
}
