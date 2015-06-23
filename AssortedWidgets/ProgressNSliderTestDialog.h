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
			Widgets::Button *closeButton;
			Widgets::Label *valueLabel;
			Widgets::ProgressBar *horizontalPBar;
			Widgets::ProgressBar *verticalPBar;
			Widgets::SlideBar *horizontalSBar;
			Widgets::SlideBar *verticalSBar;
			Layout::BorderLayout *borderLayout;
			Widgets::Panel *centerPanel;
			Layout::GirdLayout *centerGirdLayout;
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