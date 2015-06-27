#pragma once
#include "Dialog.h"
#include "TextField.h"
#include "DropList.h"
#include "GirdLayout.h"
#include "Button.h"
#include "DropListItem.h"
#include "Label.h"

namespace AssortedWidgets
{
	namespace Test
	{
		class TextNDropTestDialog:public Widgets::Dialog
		{
		private:
            Widgets::Button *m_closeButton;
            Widgets::TextField *m_textField;
            Widgets::DropList *m_dropList;
            Widgets::DropListItem *m_option1;
            Widgets::DropListItem *m_option2;
            Widgets::DropListItem *m_option3;
            Layout::GirdLayout *m_girdLayout;
            Widgets::Label *m_optionLabel;
            Widgets::Label *m_textLabel;
		public:
			void onClose(const Event::MouseEvent &e);
			TextNDropTestDialog(void);
		public:
			~TextNDropTestDialog(void);
		};
	}
}
