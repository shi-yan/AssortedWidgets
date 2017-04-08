#include "GridLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
        GridLayoutTestDialog::GridLayoutTestDialog(void):Dialog("GridLayout Test:",300,300,320,160)
		{
            m_gridLayout=new Layout::GridLayout(3,3);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);

            m_gridLayout->setHorizontalAlignment(2,2,Layout::GridLayout::HCenter);
            m_closeButton=new Widgets::Button("Close");

            m_label1=new Widgets::Label("1");
            m_label1->setHorizontalStyle(Widgets::Label::Stretch);
            m_label1->setVerticalStyle(Widgets::Label::Stretch);
            m_label1->setDrawBackground(true);

            m_label2=new Widgets::Label("2");
            m_label2->setHorizontalStyle(Widgets::Label::Stretch);
            m_label2->setVerticalStyle(Widgets::Label::Stretch);
            m_label2->setDrawBackground(true);

            m_label3=new Widgets::Label("3");
            m_label3->setHorizontalStyle(Widgets::Label::Stretch);
            m_label3->setVerticalStyle(Widgets::Label::Stretch);
            m_label3->setDrawBackground(true);

            m_label4=new Widgets::Label("4");
            m_label4->setHorizontalStyle(Widgets::Label::Stretch);
            m_label4->setVerticalStyle(Widgets::Label::Stretch);
            m_label4->setDrawBackground(true);

            m_label5=new Widgets::Label("5");
            m_label5->setHorizontalStyle(Widgets::Label::Stretch);
            m_label5->setVerticalStyle(Widgets::Label::Stretch);
            m_label5->setDrawBackground(true);

            m_label6=new Widgets::Label("6");
            m_label6->setHorizontalStyle(Widgets::Label::Stretch);
            m_label6->setVerticalStyle(Widgets::Label::Stretch);
            m_label6->setDrawBackground(true);

            m_label7=new Widgets::Label("7");
            m_label7->setHorizontalStyle(Widgets::Label::Stretch);
            m_label7->setVerticalStyle(Widgets::Label::Stretch);
            m_label7->setDrawBackground(true);

            m_label8=new Widgets::Label("8");
            m_label8->setHorizontalStyle(Widgets::Label::Stretch);
            m_label8->setVerticalStyle(Widgets::Label::Stretch);
            m_label8->setDrawBackground(true);

            setLayout(m_gridLayout);
            add(m_label1);
            add(m_label2);
            add(m_label3);
            add(m_label4);
            add(m_label5);
            add(m_label6);
            add(m_label7);
            add(m_label8);
            add(m_closeButton);

			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(GridLayoutTestDialog::onClose));
		}

        void GridLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
            (void) e;
			Close();
		}

        GridLayoutTestDialog::~GridLayoutTestDialog(void)
		{
            delete m_gridLayout;
            delete m_closeButton;
            delete m_label1;
            delete m_label2;
            delete m_label3;
            delete m_label4;
            delete m_label5;
            delete m_label6;
            delete m_label7;
            delete m_label8;
		}
	}
}
