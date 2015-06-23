#pragma once
#include "Dialog.h"
#include "BorderLayout.h"
#include "Panel.h"
#include "FlowLayout.h"
#include "GirdLayout.h"
#include "Button.h"
#include "Label.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class MultipleLayoutTestDialog:public Widgets::Dialog
		{
		private:
			Layout::GirdLayout *girdLayout;
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

			Widgets::Label *northLabel;
			Widgets::Label *southLabel;
			Widgets::Label *westLabel;
			Widgets::Label *eastLabel;
			Widgets::Label *centerLabel;
			Layout::BorderLayout *borderLayout;

			Widgets::Panel *flowPanel;
			Widgets::Panel *borderPanel;





		public:
			void onClose(const Event::MouseEvent &e);
			MultipleLayoutTestDialog(void);
		public:
			~MultipleLayoutTestDialog(void);
		};
	}
}