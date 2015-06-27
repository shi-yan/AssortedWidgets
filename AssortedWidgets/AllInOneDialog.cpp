#include "AllInOneDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
        AllInOneDialog::AllInOneDialog(void)
            :Dialog("All In One:", 450, 450, 450, 280)
		{
            m_girdLayout=new Layout::GirdLayout(4,4);
            m_girdLayout->setRight(16);
            m_girdLayout->setLeft(16);
            m_girdLayout->setTop(8);
            m_girdLayout->setBottom(8);
            m_girdLayout->setSpacer(4);

            m_label=new Widgets::Label("Widget Gallery");
            add(m_label);

            m_closeButton=new Widgets::Button("Close");
            add(m_closeButton);

            m_textField=new Widgets::TextField(100,"Text Input");
            add(m_textField);

            m_labelInScroll=new Widgets::Label("I am a Label in a Scroll Panel.");
            m_labelInScroll->size.width=500;
            m_labelInScroll->size.height=500;
            m_labelInScroll->setDrawBackground(true);
            m_scrollPanel=new Widgets::ScrollPanel();
            m_scrollPanel->setContent(m_labelInScroll);
            add(m_scrollPanel);

            m_check=new Widgets::CheckButton("Check Me");
            add(m_check);

            m_radioGroup=new Widgets::RadioGroup();
            m_radio1=new Widgets::RadioButton("Radio 1",m_radioGroup);
            m_radio2=new Widgets::RadioButton("Radio 2",m_radioGroup);
            add(m_radio1);
            add(m_radio2);

            m_sliderH=new Widgets::SlideBar();
            add(m_sliderH);

            m_sliderV=new Widgets::SlideBar(Widgets::SlideBar::Vertical);
            add(m_sliderV);

            m_progressH=new Widgets::ProgressBar();
            m_progressH->setValue(60.0f);
            add(m_progressH);

            m_progressV=new Widgets::ProgressBar(Widgets::ProgressBar::Vertical);
            m_progressV->setValue(50.0f);
            add(m_progressV);

            m_option1=new Widgets::DropListItem("Option 1");
            m_option2=new Widgets::DropListItem("Option 2");
            m_option3=new Widgets::DropListItem("Option 3");

            m_option4=new Widgets::DropListItem("Google");
            m_option5=new Widgets::DropListItem("Yahoo!");
            m_option6=new Widgets::DropListItem("Microsoft");

            m_dropList1=new Widgets::DropList();
            m_dropList1->add(m_option1);
            m_dropList1->add(m_option2);
            m_dropList1->add(m_option3);
            add(m_dropList1);

            m_dropList2=new Widgets::DropList();
            m_dropList2->add(m_option4);
            m_dropList2->add(m_option5);
            m_dropList2->add(m_option6);
            add(m_dropList2);

            setLayout(m_girdLayout);
			pack();
            MouseDelegate onClose;
			onClose.bind(this,&AllInOneDialog::onClose);
            m_closeButton->mouseReleasedHandlerList.push_back(onClose);
		}

        void AllInOneDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		AllInOneDialog::~AllInOneDialog(void)
		{
            delete m_label;
            delete m_closeButton;
            delete m_textField;
            delete m_scrollPanel;
            delete m_labelInScroll;
            delete m_check;
            delete m_radio1;
            delete m_radio2;
            delete m_sliderH;
            delete m_sliderV;
            delete m_progressH;
            delete m_progressV;
            delete m_dropList1;
            delete m_dropList2;

            delete m_radioGroup;
            delete m_option1;
            delete m_option2;
            delete m_option3;

            delete m_option4;
            delete m_option5;
            delete m_option6;

            delete m_girdLayout;
		}
	}
}
