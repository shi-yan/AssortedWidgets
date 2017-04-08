#include "DialogTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		DialogTestDialog::DialogTestDialog(void):Dialog("Dialog Test:",500,500,260,180)
		{
            m_gridLayout=new Layout::GridLayout(4,1);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);

            m_gridLayout->setHorizontalAlignment(1,0,Layout::GridLayout::HCenter);
            m_gridLayout->setHorizontalAlignment(2,0,Layout::GridLayout::HCenter);
            m_gridLayout->setHorizontalAlignment(3,0,Layout::GridLayout::HRight);

            m_closeButton=new Widgets::Button("Close");

            m_dragAble=new Widgets::CheckButton("Dragable",true);
            m_sizeAble=new Widgets::CheckButton("Resizable",true);

            m_label=new Widgets::Label("This is a modal dialog.");

            setLayout(m_gridLayout);

            add(m_label);
            add(m_dragAble);
            add(m_sizeAble);
            add(m_closeButton);

			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(DialogTestDialog::onClose));
            m_dragAble->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(DialogTestDialog::onDrag));
            m_sizeAble->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(DialogTestDialog::onSize));
		}

		void DialogTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

        void DialogTestDialog::onDrag(const Event::MouseEvent &)
		{
            if(m_dragAble->isCheck()==true)
			{
				setDragable(true);
			}
			else
			{
				setDragable(false);
			}
		}

        void DialogTestDialog::onSize(const Event::MouseEvent &)
		{
            if(m_sizeAble->isCheck())
			{
				setResizable(true);
			}
			else
			{
				setResizable(false);
			}
		}

		DialogTestDialog::~DialogTestDialog(void)
		{
            delete m_label;
            delete m_closeButton;
            delete m_gridLayout;
            delete m_dragAble;
            delete m_sizeAble;
		}
	}
}
