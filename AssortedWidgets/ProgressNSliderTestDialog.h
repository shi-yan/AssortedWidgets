#pragma once
#include "Dialog.h"
#include "BorderLayout.h"
#include "ProgressBar.h"
#include "SlideBar.h"
#include "Button.h"
#include "Label.h"
#include "Panel.h"
#include "GirdLayout.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class ProgressNSliderTestDialog:public Widgets::Dialog
		{
		private:
            Widgets::Button *m_closeButton;
            Widgets::Label *m_valueLabel;
            Widgets::ProgressBar *m_horizontalPBar;
            Widgets::ProgressBar *m_verticalPBar;
            Widgets::SlideBar *m_horizontalSBar;
            Widgets::SlideBar *m_verticalSBar;
            Layout::BorderLayout *m_borderLayout;
            Widgets::Panel *m_centerPanel;
            Layout::GirdLayout *m_centerGirdLayout;
		public:
			void onClose(const Event::MouseEvent &e);
			void onHSlider(const Event::MouseEvent &e);
			void onVSlider(const Event::MouseEvent &e);
			ProgressNSliderTestDialog(void);
		public:
			~ProgressNSliderTestDialog(void);
		};
	}
}
