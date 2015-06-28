#include "LabelNButtonTestDialog.h"


namespace AssortedWidgets
{
	namespace Test
	{
		LabelNButtonTestDialog::LabelNButtonTestDialog(void):Dialog("Label and Button Test:",50,50,320,140)
		{
            m_girdLayout=new Layout::GirdLayout(3,1);
            m_girdLayout->setHorizontalAlignment(0,0,Layout::GirdLayout::HLeft);
            m_girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
            m_girdLayout->setHorizontalAlignment(2,0,Layout::GirdLayout::HRight);
            m_girdLayout->setRight(16);
            m_girdLayout->setLeft(16);
            m_girdLayout->setTop(8);
            m_girdLayout->setBottom(8);
            m_girdLayout->setSpacer(4);
            m_testLabel=new Widgets::Label("This is a Label test.");
            m_testButton=new Widgets::Button("This is a Button test.");
            m_closeButton=new Widgets::Button("Close");
            add(m_testLabel);
            add(m_testButton);
            add(m_closeButton);
            setLayout(m_girdLayout);
			
			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(LabelNButtonTestDialog::onClose));
		}

        void LabelNButtonTestDialog::onClose(const Event::MouseEvent &)
		{
			Close();
        }

		LabelNButtonTestDialog::~LabelNButtonTestDialog(void)
		{
            delete m_testLabel;
            delete m_closeButton;
            delete m_testButton;
            delete m_girdLayout;
		}
	}
}
