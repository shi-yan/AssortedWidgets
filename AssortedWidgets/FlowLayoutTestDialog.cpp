#include "FlowLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		FlowLayoutTestDialog::FlowLayoutTestDialog(void):Dialog("FlowLayout Test:",250,250,200,180)
		{
			flowLayout=new Layout::FlowLayout(16,16,16,16,4);
			closeButton=new Widgets::Button("Close");

			TheLabel=new Widgets::Label("The");
			TheLabel->setDrawBackground(true);

			quickLabel=new Widgets::Label("quick");
			quickLabel->setDrawBackground(true);

			brownLabel=new Widgets::Label("brown");
			brownLabel->setDrawBackground(true);

			foxLabel=new Widgets::Label("Fox");
			foxLabel->setDrawBackground(true);

			jumpsLabel=new Widgets::Label("jumps");
			jumpsLabel->setDrawBackground(true);

			overLabel=new Widgets::Label("over");
			overLabel->setDrawBackground(true);

			theLabel=new Widgets::Label("a");
			theLabel->setDrawBackground(true);

			lazyDogLabel=new Widgets::Label("lazy dog.");
			lazyDogLabel->setDrawBackground(true);

			setLayout(flowLayout);

			add(TheLabel);
			add(quickLabel);
			add(brownLabel);
			add(foxLabel);
			add(jumpsLabel);
			add(overLabel);
			add(theLabel);
			add(lazyDogLabel);
			add(closeButton);

			pack();

            closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(FlowLayoutTestDialog::onClose));

		}

		void FlowLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		FlowLayoutTestDialog::~FlowLayoutTestDialog(void)
		{
			delete flowLayout;
			delete closeButton;
			delete TheLabel;
			delete quickLabel;
			delete brownLabel;
			delete foxLabel;
			delete jumpsLabel;
			delete overLabel;
			delete theLabel;
			delete lazyDogLabel;
		}
	}
}
