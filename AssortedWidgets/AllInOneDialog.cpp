#include "AllInOneDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		AllInOneDialog::AllInOneDialog(void):Dialog("All In One:",450,450,450,280)
		{
			girdLayout=new Layout::GirdLayout(4,4);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			label=new Widgets::Label("Widget Gallery");
			add(label);

			closeButton=new Widgets::Button("Close");
			add(closeButton);

			textField=new Widgets::TextField(100,"Text Input");
			add(textField);

			labelInScroll=new Widgets::Label("I am a Label in a Scroll Panel.");
			labelInScroll->size.width=500;
			labelInScroll->size.height=500;
			labelInScroll->setDrawBackground(true);
			scrollPanel=new Widgets::ScrollPanel();
			scrollPanel->setContent(labelInScroll);
			add(scrollPanel);

			check=new Widgets::CheckButton("Check Me");
			add(check);

			radioGroup=new Widgets::RadioGroup();
			radio1=new Widgets::RadioButton("Radio 1",radioGroup);
			radio2=new Widgets::RadioButton("Radio 2",radioGroup);
			add(radio1);
			add(radio2);

			sliderH=new Widgets::SlideBar();
			add(sliderH);

			sliderV=new Widgets::SlideBar(Widgets::SlideBar::Vertical);
			add(sliderV);

			progressH=new Widgets::ProgressBar();
			progressH->setValue(60.0f);
			add(progressH);

			progressV=new Widgets::ProgressBar(Widgets::ProgressBar::Vertical);
			progressV->setValue(50.0f);
			add(progressV);

			option1=new Widgets::DropListItem("Option 1");
			option2=new Widgets::DropListItem("Option 2");
			option3=new Widgets::DropListItem("Option 3");

			option4=new Widgets::DropListItem("Google");
			option5=new Widgets::DropListItem("Yahoo!");
			option6=new Widgets::DropListItem("Microsoft");

			dropList1=new Widgets::DropList();
			dropList1->add(option1);
			dropList1->add(option2);
			dropList1->add(option3);
			add(dropList1);

			dropList2=new Widgets::DropList();
			dropList2->add(option4);
			dropList2->add(option5);
			dropList2->add(option6);
			add(dropList2);

			setLayout(girdLayout);
			pack();
								MouseDelegate onClose;
			onClose.bind(this,&AllInOneDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);

		}

						void AllInOneDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		AllInOneDialog::~AllInOneDialog(void)
		{
			delete label;
			delete closeButton;
			delete textField;
			delete scrollPanel;
			delete labelInScroll;
			delete check;
			delete radio1;
			delete radio2;
			delete sliderH;
			delete sliderV;
			delete progressH;
			delete progressV;
			delete dropList1;
			delete dropList2;

			delete radioGroup;
			delete option1;
			delete option2;
			delete option3;

			delete option4;
			delete option5;
			delete option6;

			delete girdLayout;
		}
	}
}