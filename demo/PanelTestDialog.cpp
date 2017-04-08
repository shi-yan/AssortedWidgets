#include "PanelTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		PanelTestDialog::PanelTestDialog(void):Dialog("Scroll Panel Test:",400,400,320,240)
		{
            m_gridLayout=new Layout::GridLayout(2,1);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);

            m_gridLayout->setHorizontalAlignment(1,0,Layout::GridLayout::HRight);

            m_closeButton=new Widgets::Button("Close");
            m_label=new Widgets::Label("I am a very very big Label in a Scroll Panel.");
            m_label->m_size.m_height=m_label->m_size.m_width=500;
            m_panel=new Widgets::ScrollPanel();
            m_panel->setContent(m_label);

            setLayout(m_gridLayout);
            add(m_panel);
            add(m_closeButton);

            pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(PanelTestDialog::onClose));
		}

        void PanelTestDialog::onClose(const Event::MouseEvent &)
		{
			Close();
		}

		PanelTestDialog::~PanelTestDialog(void)
		{
            delete m_closeButton;
            delete m_label;
            delete m_panel;
            delete m_gridLayout;
		}
	}
}
