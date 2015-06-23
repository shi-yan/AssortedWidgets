#include "BorderLayoutTestDialog.h"
namespace AssortedWidgets
{
	namespace Test
	{
		BorderLayoutTestDialog::BorderLayoutTestDialog(void):Dialog("BorderLayout Test:",250,250,320,240)
		{
			borderLayout=new Layout::BorderLayout(16,16,16,16,4);
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

			setLayout(borderLayout);

			add(northLabel);
			add(southLabel);
			add(closeButton);
			add(westLabel);
			add(eastLabel);
			add(centerLabel);

			pack();

						MouseDelegate onClose;
			onClose.bind(this,&BorderLayoutTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);


		}

				void BorderLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		BorderLayoutTestDialog::~BorderLayoutTestDialog(void)
		{
			delete closeButton;
			delete northLabel;
			delete southLabel;
			delete westLabel;
			delete eastLabel;
			delete borderLayout;
			delete centerLabel;
		}
	}
}