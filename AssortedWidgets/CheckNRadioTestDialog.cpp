#include "CheckNRadioTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		CheckNRadioTestDialog::CheckNRadioTestDialog(void):Dialog("Check And Radio Test:",100,100,320,200)
		{
			girdLayout=new Layout::GirdLayout(4,2);

			girdLayout->setHorizontalAlignment(0,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(2,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(3,0,Layout::GirdLayout::HCenter);

			girdLayout->setHorizontalAlignment(0,1,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(1,1,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(2,1,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(3,1,Layout::GirdLayout::HCenter);

			girdLayout->setVerticalAlignment(0,0,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(1,0,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(2,0,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(3,0,Layout::GirdLayout::VCenter);

			girdLayout->setVerticalAlignment(0,1,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(1,1,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(2,1,Layout::GirdLayout::VCenter);
			girdLayout->setVerticalAlignment(3,1,Layout::GirdLayout::VCenter);

			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			closeButton=new Widgets::Button("Close");
			checkButton1=new Widgets::CheckButton("Check 1");
			checkButton2=new Widgets::CheckButton("Check 2");
			checkButton3=new Widgets::CheckButton("Check 3");
			radioGroup=new Widgets::RadioGroup();
			radioButton1=new Widgets::RadioButton("Radio 1",radioGroup);
			radioButton2=new Widgets::RadioButton("Radio 2",radioGroup);
			radioButton3=new Widgets::RadioButton("Radio 3",radioGroup);
			spacer=new Widgets::Spacer(Widgets::Spacer::Fit);

			add(checkButton1);
			add(radioButton1);
			add(checkButton2);
			add(radioButton2);
			add(checkButton3);
			add(radioButton3);
			add(spacer);
			add(closeButton);
			setLayout(girdLayout);
			pack();

			
			MouseDelegate onClose;
			onClose.bind(this,&CheckNRadioTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);
		}

		void CheckNRadioTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		CheckNRadioTestDialog::~CheckNRadioTestDialog(void)
		{
			delete closeButton;
			delete checkButton1;
			delete checkButton2;
			delete checkButton3;
			delete radioButton1;
			delete radioButton2;
			delete radioButton3;
			delete radioGroup;
			delete spacer;
			delete girdLayout;
		}
	}
}