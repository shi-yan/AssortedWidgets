#include "FlowLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		FlowLayoutTestDialog::FlowLayoutTestDialog(void):Dialog("FlowLayout Test:",250,250,200,180)
		{
            m_flowLayout=new Layout::FlowLayout(16,16,16,16,4);
            m_closeButton=new Widgets::Button("Close");

            m_TheLabel=new Widgets::Label("The");
            m_TheLabel->setDrawBackground(true);

            m_quickLabel=new Widgets::Label("quick");
            m_quickLabel->setDrawBackground(true);

            m_brownLabel=new Widgets::Label("brown");
            m_brownLabel->setDrawBackground(true);

            m_foxLabel=new Widgets::Label("Fox");
            m_foxLabel->setDrawBackground(true);

            m_jumpsLabel=new Widgets::Label("jumps");
            m_jumpsLabel->setDrawBackground(true);

            m_overLabel=new Widgets::Label("over");
            m_overLabel->setDrawBackground(true);

            m_aLabel=new Widgets::Label("a");
            m_aLabel->setDrawBackground(true);

            m_lazyDogLabel=new Widgets::Label("lazy dog.");
            m_lazyDogLabel->setDrawBackground(true);

            setLayout(m_flowLayout);

            add(m_TheLabel);
            add(m_quickLabel);
            add(m_brownLabel);
            add(m_foxLabel);
            add(m_jumpsLabel);
            add(m_overLabel);
            add(m_aLabel);
            add(m_lazyDogLabel);
            add(m_closeButton);

			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(FlowLayoutTestDialog::onClose));
		}

        void FlowLayoutTestDialog::onClose(const Event::MouseEvent &)
		{
			Close();
		}

		FlowLayoutTestDialog::~FlowLayoutTestDialog(void)
		{
            delete m_flowLayout;
            delete m_closeButton;
            delete m_TheLabel;
            delete m_quickLabel;
            delete m_brownLabel;
            delete m_foxLabel;
            delete m_jumpsLabel;
            delete m_overLabel;
            delete m_aLabel;
            delete m_lazyDogLabel;
		}
	}
}
