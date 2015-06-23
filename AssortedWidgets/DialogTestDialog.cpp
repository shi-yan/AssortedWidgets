#include "DialogTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		DialogTestDialog::DialogTestDialog(void):Dialog("Dialog Test:",500,500,260,180)
		{
			girdLayout=new Layout::GirdLayout(4,1);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(2,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(3,0,Layout::GirdLayout::HRight);

			closeButton=new Widgets::Button("Close");

			dragAble=new Widgets::CheckButton("Dragable",true);
			sizeAble=new Widgets::CheckButton("Resizable",true);

			label=new Widgets::Label("This is a modal dialog.");

			setLayout(girdLayout);

			add(label);
			add(dragAble);
			add(sizeAble);
			add(closeButton);

			pack();

			MouseDelegate onClose;
			onClose.bind(this,&DialogTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);

			MouseDelegate onDrag;
			onDrag.bind(this,&DialogTestDialog::onDrag);
			dragAble->mouseReleasedHandlerList.push_back(onDrag);

			MouseDelegate onSize;
			onSize.bind(this,&DialogTestDialog::onSize);
			sizeAble->mouseReleasedHandlerList.push_back(onSize);

		}

		void DialogTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		void DialogTestDialog::onDrag(const Event::MouseEvent &e)
		{
			if(dragAble->isCheck()==true)
			{
				setDragable(true);
			}
			else
			{
				setDragable(false);
			}
		}

		void DialogTestDialog::onSize(const Event::MouseEvent &e)
		{
			if(sizeAble->isCheck())
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
			delete label;
			delete closeButton;
			delete girdLayout;
			delete dragAble;
			delete sizeAble;
		
		}
	}
}