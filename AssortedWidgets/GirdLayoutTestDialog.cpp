#include "GirdLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		GirdLayoutTestDialog::GirdLayoutTestDialog(void):Dialog("GirdLayout Test:",300,300,320,160)
		{
			girdLayout=new Layout::GirdLayout(3,3);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			girdLayout->setHorizontalAlignment(2,2,Layout::GirdLayout::HCenter);
			closeButton=new Widgets::Button("Close");

			label1=new Widgets::Label("1");
			label1->setHorizontalStyle(Widgets::Label::Stretch);
			label1->setVerticalStyle(Widgets::Label::Stretch);
			label1->setDrawBackground(true);

			label2=new Widgets::Label("2");
			label2->setHorizontalStyle(Widgets::Label::Stretch);
			label2->setVerticalStyle(Widgets::Label::Stretch);
			label2->setDrawBackground(true);

			label3=new Widgets::Label("3");
			label3->setHorizontalStyle(Widgets::Label::Stretch);
			label3->setVerticalStyle(Widgets::Label::Stretch);
			label3->setDrawBackground(true);

			label4=new Widgets::Label("4");
			label4->setHorizontalStyle(Widgets::Label::Stretch);
			label4->setVerticalStyle(Widgets::Label::Stretch);
			label4->setDrawBackground(true);

			label5=new Widgets::Label("5");
			label5->setHorizontalStyle(Widgets::Label::Stretch);
			label5->setVerticalStyle(Widgets::Label::Stretch);
			label5->setDrawBackground(true);

			label6=new Widgets::Label("6");
			label6->setHorizontalStyle(Widgets::Label::Stretch);
			label6->setVerticalStyle(Widgets::Label::Stretch);
			label6->setDrawBackground(true);

			label7=new Widgets::Label("7");
			label7->setHorizontalStyle(Widgets::Label::Stretch);
			label7->setVerticalStyle(Widgets::Label::Stretch);
			label7->setDrawBackground(true);

			label8=new Widgets::Label("8");
			label8->setHorizontalStyle(Widgets::Label::Stretch);
			label8->setVerticalStyle(Widgets::Label::Stretch);
			label8->setDrawBackground(true);

			setLayout(girdLayout);
			add(label1);
			add(label2);
			add(label3);
			add(label4);
			add(label5);
			add(label6);
			add(label7);
			add(label8);
			add(closeButton);

			pack();

            closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(GirdLayoutTestDialog::onClose));

		}

        void GirdLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		GirdLayoutTestDialog::~GirdLayoutTestDialog(void)
		{
			delete girdLayout;
			delete closeButton;
			delete label1;
			delete label2;
			delete label3;
			delete label4;
			delete label5;
			delete label6;
			delete label7;
			delete label8;
		}
	}
}
