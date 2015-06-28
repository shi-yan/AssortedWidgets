#include "CheckNRadioTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		CheckNRadioTestDialog::CheckNRadioTestDialog(void):Dialog("Check And Radio Test:",100,100,320,200)
		{
            m_girdLayout=new Layout::GirdLayout(4,2);

            m_girdLayout->setHorizontalAlignment(0,0,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(2,0,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(3,0,Layout::GirdLayout::HCenter);

            m_girdLayout->setHorizontalAlignment(0,1,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(1,1,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(2,1,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(3,1,Layout::GirdLayout::HCenter);

            m_girdLayout->setVerticalAlignment(0,0,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(1,0,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(2,0,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(3,0,Layout::GirdLayout::VCenter);

            m_girdLayout->setVerticalAlignment(0,1,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(1,1,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(2,1,Layout::GirdLayout::VCenter);
            m_girdLayout->setVerticalAlignment(3,1,Layout::GirdLayout::VCenter);

            m_girdLayout->setRight(16);
            m_girdLayout->setLeft(16);
            m_girdLayout->setTop(8);
            m_girdLayout->setBottom(8);
            m_girdLayout->setSpacer(4);

            m_closeButton=new Widgets::Button("Close");
            m_checkButton1=new Widgets::CheckButton("Check 1");
            m_checkButton2=new Widgets::CheckButton("Check 2");
            m_checkButton3=new Widgets::CheckButton("Check 3");
            m_radioGroup=new Widgets::RadioGroup();
            m_radioButton1=new Widgets::RadioButton("Radio 1",m_radioGroup);
            m_radioButton2=new Widgets::RadioButton("Radio 2",m_radioGroup);
            m_radioButton3=new Widgets::RadioButton("Radio 3",m_radioGroup);
            m_spacer=new Widgets::Spacer(Widgets::Spacer::Fit);

            add(m_checkButton1);
            add(m_radioButton1);
            add(m_checkButton2);
            add(m_radioButton2);
            add(m_checkButton3);
            add(m_radioButton3);
            add(m_spacer);
            add(m_closeButton);
            setLayout(m_girdLayout);
			pack();


            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(CheckNRadioTestDialog::onClose));
		}

		void CheckNRadioTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		CheckNRadioTestDialog::~CheckNRadioTestDialog(void)
		{
            delete m_closeButton;
            delete m_checkButton1;
            delete m_checkButton2;
            delete m_checkButton3;
            delete m_radioButton1;
            delete m_radioButton2;
            delete m_radioButton3;
            delete m_radioGroup;
            delete m_spacer;
            delete m_girdLayout;
		}
	}
}
