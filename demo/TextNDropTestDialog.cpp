#include "TextNDropTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
        TextNDropTestDialog::TextNDropTestDialog(void)
            :Dialog("TextField and DropList Test:",200,200,320,200)
		{
            m_gridLayout=new Layout::GridLayout(5,1);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);

            m_gridLayout->setHorizontalAlignment(1,0,Layout::GridLayout::HCenter);
            m_gridLayout->setHorizontalAlignment(3,0,Layout::GridLayout::HCenter);
            m_gridLayout->setHorizontalAlignment(4,0,Layout::GridLayout::HRight);

            m_closeButton=new Widgets::Button("Close");
            m_textField=new Widgets::TextField(160);
            m_dropList=new Widgets::DropList();
            m_option1=new Widgets::DropListItem("Option one");
            m_option2=new Widgets::DropListItem("Option Two");
            m_option3=new Widgets::DropListItem("Option Three");
            m_dropList->add(m_option1);
            m_dropList->add(m_option2);
            m_dropList->add(m_option3);

            m_textLabel=new Widgets::Label("Text input here:");
            m_optionLabel=new Widgets::Label("Drop List test:");

            setLayout(m_gridLayout);

            add(m_textLabel);
            add(m_textField);
            add(m_optionLabel);
            add(m_dropList);
            add(m_closeButton);

			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(TextNDropTestDialog::onClose));
		}

		void TextNDropTestDialog::onClose(const Event::MouseEvent &e)
		{
            (void) e;
			Close();
		}

		TextNDropTestDialog::~TextNDropTestDialog(void)
		{
            delete m_closeButton;
            delete m_textField;
            delete m_dropList;
            delete m_option1;
            delete m_option2;
            delete m_option3;
            delete m_gridLayout;
            delete m_optionLabel;
            delete m_textLabel;
		}
	}
}
