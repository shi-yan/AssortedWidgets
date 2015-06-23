#include "ProgressNSliderTestDialog.h"
#include <iostream>     
#include <sstream>     
#include <iomanip>

namespace AssortedWidgets
{
	namespace Test
	{
		ProgressNSliderTestDialog::ProgressNSliderTestDialog(void):Dialog("Progress and Slider Test:",150,150,320,200)
		{
			borderLayout=new Layout::BorderLayout(16,16,16,16,8);
			borderLayout->setSouthHAlignment(Layout::BorderLayout::HRight);

			closeButton=new Widgets::Button("Close");
			closeButton->setLayoutProperty(Layout::BorderLayout::South);

			valueLabel=new Widgets::Label("Value:0%");
			valueLabel->setLayoutProperty(Layout::BorderLayout::North);

			centerPanel=new Widgets::Panel();
			centerGirdLayout=new Layout::GirdLayout(2,1);
			centerPanel->setLayout(centerGirdLayout);

			horizontalPBar=new Widgets::ProgressBar(0.0f,100.0f,0.0f);
			horizontalSBar=new Widgets::SlideBar(0.0f,100.0f,0.0f);

			centerPanel->add(horizontalPBar);
			centerPanel->add(horizontalSBar);
			centerPanel->setLayoutProperty(Layout::BorderLayout::Center);
			centerPanel->pack();

			verticalPBar=new Widgets::ProgressBar(0.0f,100.0f,0.0f,Widgets::ProgressBar::Vertical);
			verticalSBar=new Widgets::SlideBar(0.0f,100.0f,0.0f,Widgets::ProgressBar::Vertical);

			verticalPBar->setLayoutProperty(Layout::BorderLayout::East);
			verticalSBar->setLayoutProperty(Layout::BorderLayout::East);

			add(closeButton);
			add(valueLabel);
			add(centerPanel);
			add(verticalPBar);
			add(verticalSBar);

			setLayout(borderLayout);

			pack();

			MouseDelegate onClose;
			onClose.bind(this,&ProgressNSliderTestDialog::onClose);
			closeButton->mouseReleasedHandlerList.push_back(onClose);

			MouseDelegate onHSlider;
			onHSlider.bind(this,&ProgressNSliderTestDialog::onHSlider);
			horizontalSBar->mouseReleasedHandlerList.push_back(onHSlider);

			MouseDelegate onVSlider;
			onVSlider.bind(this,&ProgressNSliderTestDialog::onVSlider);
			verticalSBar->mouseReleasedHandlerList.push_back(onVSlider);

		}

		void ProgressNSliderTestDialog::onHSlider(const Event::MouseEvent &e)
		{
			horizontalPBar->setValue(horizontalSBar->getValue());
			std::ostringstream ostr ;
			ostr<<"Value:"<<static_cast<int>(horizontalSBar->getValue())<<"%";     
			valueLabel->setText(ostr.str());
		}

		void ProgressNSliderTestDialog::onVSlider(const Event::MouseEvent &e)
		{
			verticalPBar->setValue(verticalSBar->getValue());
		}

		void ProgressNSliderTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		ProgressNSliderTestDialog::~ProgressNSliderTestDialog(void)
		{
			delete closeButton;
			delete valueLabel;
			delete horizontalPBar;
			delete verticalPBar;
			delete horizontalSBar;
			delete verticalSBar;
			delete borderLayout;
			delete centerPanel;
			delete centerGirdLayout;
		}
	}
}