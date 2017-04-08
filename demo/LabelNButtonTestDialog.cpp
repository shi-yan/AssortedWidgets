#include "LabelNButtonTestDialog.h"


namespace AssortedWidgets
{
	namespace Test
	{
		LabelNButtonTestDialog::LabelNButtonTestDialog(void):Dialog("Label and Button Test:",50,50,320,140)
		{
            m_gridLayout=new Layout::GridLayout(3,1);
            m_gridLayout->setHorizontalAlignment(0,0,Layout::GridLayout::HLeft);
            m_gridLayout->setHorizontalAlignment(1,0,Layout::GridLayout::HCenter);
            m_gridLayout->setHorizontalAlignment(2,0,Layout::GridLayout::HRight);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);
            m_testLabel=new Widgets::Label("This is a Label test.");
            m_testButton=new Widgets::Button("This is a Button test.");
            m_closeButton=new Widgets::Button("Close");
            add(m_testLabel);
            add(m_testButton);
            add(m_closeButton);
            setLayout(m_gridLayout);
			
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
            delete m_gridLayout;
		}
	}
}
