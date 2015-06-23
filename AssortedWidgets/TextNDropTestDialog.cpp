#include "TextNDropTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		TextNDropTestDialog::TextNDropTestDialog(void):Dialog("TextField and DropList Test:",200,200,320,200)
		{
			girdLayout=new Layout::GirdLayout(5,1);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			girdLayout->setHorizontalAlignment(1,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(3,0,Layout::GirdLayout::HCenter);
			girdLayout->setHorizontalAlignment(4,0,Layout::GirdLayout::HRight);

			closeButton=new Widgets::Button("Close");
			textField=new Widgets::TextField(160);
			dropList=new Widgets::DropList();
			option1=new Widgets::DropListItem("Option one");
			option2=new Widgets::DropListItem("Option Two");
			option3=new Widgets::DropListItem("Option Three");
			dropList->add(option1);
			dropList->add(option2);
			dropList->add(option3);

			textLabel=new Widgets::Label("Text input here:");
			optionLabel=new Widgets::Label("Drop List test:");

			setLayout(girdLayout);

			add(textLabel);
			add(textField);
			add(optionLabel);
			add(dropList);
			add(closeButton);

			pack();

			MouseDelegate onClose;
			onClose.bind(this,&TextNDropTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);


		}

		void TextNDropTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		TextNDropTestDialog::~TextNDropTestDialog(void)
		{
			delete closeButton;
			delete textField;
			delete dropList;
			delete option1;
			delete option2;
			delete option3;
			delete girdLayout;
			delete optionLabel;
			delete textLabel;
		}
	}
}