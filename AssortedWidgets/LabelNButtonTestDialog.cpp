#include "LabelNButtonTestDialog.h"


namespace AssortedWidgets
{
	namespace Test
	{
		LabelNButtonTestDialog::LabelNButtonTestDialog(void):Dialog("Label and Button Test:",50,50,320,140)
		{
			girdLayout=new Layout::GirdLayout(3,1);
			girdLayout->setHorizontalAlignment(0,0,Layout::GirdLayout::HLeft);
			girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(2,0,Layout::GirdLayout::HRight);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);
			testLabel=new Widgets::Label("This is a Label test.");
			testButton=new Widgets::Button("This is a Button test.");
			closeButton=new Widgets::Button("Close");
			add(testLabel);
			add(testButton);
			add(closeButton);
			setLayout(girdLayout);
			
			pack();

            closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(LabelNButtonTestDialog::onClose));
		}

		void LabelNButtonTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		};

		LabelNButtonTestDialog::~LabelNButtonTestDialog(void)
		{
			delete testLabel;
			delete closeButton;
			delete testButton;
			delete girdLayout;
		}
	}
}
