#include "PanelTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		PanelTestDialog::PanelTestDialog(void):Dialog("Scroll Panel Test:",400,400,320,240)
		{
			girdLayout=new Layout::GirdLayout(2,1);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HRight);

			closeButton=new Widgets::Button("Close");
			label=new Widgets::Label("I am a very very big Label in a Scroll Panel.");
			label->size.height=label->size.width=500;
			panel=new Widgets::ScrollPanel();
			panel->setContent(label);

			setLayout(girdLayout);
			add(panel);
			add(closeButton);

			pack();
								MouseDelegate onClose;
			onClose.bind(this,&PanelTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);


		}

								void PanelTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		PanelTestDialog::~PanelTestDialog(void)
		{
			delete closeButton;
			delete label;
			delete panel;
			delete girdLayout;
		}
	}
}