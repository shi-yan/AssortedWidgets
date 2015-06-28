#include "MultipleLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		MultipleLayoutTestDialog::MultipleLayoutTestDialog(void):Dialog("MultipleLayout Test:",350,350,400,180)
		{
			girdLayout=new Layout::GirdLayout(1,2);
			girdLayout->setRight(16);
			girdLayout->setLeft(16);
			girdLayout->setTop(8);
			girdLayout->setBottom(8);
			girdLayout->setSpacer(4);

			flowLayout=new Layout::FlowLayout(2,2,2,2,4);
			

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

			flowPanel=new Widgets::Panel();
			flowPanel->setLayout(flowLayout);
			flowPanel->add(TheLabel);
			flowPanel->add(quickLabel);
			flowPanel->add(brownLabel);
			flowPanel->add(foxLabel);
			flowPanel->add(jumpsLabel);
			flowPanel->add(overLabel);
			flowPanel->add(theLabel);
			flowPanel->add(lazyDogLabel);
		
			flowPanel->pack();

			borderLayout=new Layout::BorderLayout(2,2,2,2,4);
			closeButton=new Widgets::Button("Close");
			closeButton->setLayoutProperty(Layout::BorderLayout::South);

			northLabel=new Widgets::Label("North");
			northLabel->setHorizontalStyle(Widgets::Label::Stretch);
			northLabel->setDrawBackground(true);
			northLabel->setLayoutProperty(Layout::BorderLayout::North);

			southLabel=new Widgets::Label("South");
			southLabel->setHorizontalStyle(Widgets::Label::Stretch);
			southLabel->setDrawBackground(true);
			southLabel->setLayoutProperty(Layout::BorderLayout::South);

			westLabel=new Widgets::Label("West");
			westLabel->setVerticalStyle(Widgets::Label::Stretch);
			westLabel->setDrawBackground(true);
			westLabel->setLayoutProperty(Layout::BorderLayout::West);

			eastLabel=new Widgets::Label("East");
			eastLabel->setVerticalStyle(Widgets::Label::Stretch);
			eastLabel->setDrawBackground(true);
			eastLabel->setLayoutProperty(Layout::BorderLayout::East);

			centerLabel=new Widgets::Label("Center");
			centerLabel->setHorizontalStyle(Widgets::Label::Stretch);
			centerLabel->setVerticalStyle(Widgets::Label::Stretch);
			centerLabel->setDrawBackground(true);
			centerLabel->setLayoutProperty(Layout::BorderLayout::Center);

			borderPanel=new Widgets::Panel();
			borderPanel->setLayout(borderLayout);

			borderPanel->add(northLabel);
			borderPanel->add(southLabel);
			borderPanel->add(closeButton);
			borderPanel->add(westLabel);
			borderPanel->add(eastLabel);
			borderPanel->add(centerLabel);

			borderPanel->pack();
			
			setLayout(girdLayout);
			add(flowPanel);
			add(borderPanel);
			pack();

            closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MultipleLayoutTestDialog::onClose));
		}

		void MultipleLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		MultipleLayoutTestDialog::~MultipleLayoutTestDialog(void)
		{
			delete closeButton;
			delete northLabel;
			delete southLabel;
			delete westLabel;
			delete eastLabel;
			delete borderLayout;
			delete centerLabel;
			delete flowLayout;
			delete TheLabel;
			delete quickLabel;
			delete brownLabel;
			delete foxLabel;
			delete jumpsLabel;
			delete overLabel;
			delete theLabel;
			delete lazyDogLabel;
			delete girdLayout;
			delete flowPanel;
			delete borderPanel;
		}
}
}
