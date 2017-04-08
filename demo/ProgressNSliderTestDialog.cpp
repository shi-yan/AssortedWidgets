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
            m_borderLayout=new Layout::BorderLayout(16,16,16,16,8);
            m_borderLayout->setSouthHAlignment(Layout::BorderLayout::HRight);

            m_closeButton=new Widgets::Button("Close");
            m_closeButton->setLayoutProperty(Layout::BorderLayout::South);

            m_valueLabel=new Widgets::Label("Value:0%");
            m_valueLabel->setLayoutProperty(Layout::BorderLayout::North);

            m_centerPanel=new Widgets::Panel();
            m_centerGridLayout=new Layout::GridLayout(2,1);
            m_centerPanel->setLayout(m_centerGridLayout);

            m_horizontalPBar=new Widgets::ProgressBar(0.0f,100.0f,0.0f);
            m_horizontalSBar=new Widgets::SlideBar(0.0f,100.0f,0.0f);

            m_centerPanel->add(m_horizontalPBar);
            m_centerPanel->add(m_horizontalSBar);
            m_centerPanel->setLayoutProperty(Layout::BorderLayout::Center);
            m_centerPanel->pack();

            m_verticalPBar=new Widgets::ProgressBar(0.0f,100.0f,0.0f,Widgets::ProgressBar::Vertical);
            m_verticalSBar=new Widgets::SlideBar(0.0f,100.0f,0.0f,Widgets::ProgressBar::Vertical);

            m_verticalPBar->setLayoutProperty(Layout::BorderLayout::East);
            m_verticalSBar->setLayoutProperty(Layout::BorderLayout::East);

            add(m_closeButton);
            add(m_valueLabel);
            add(m_centerPanel);
            add(m_verticalPBar);
            add(m_verticalSBar);

            setLayout(m_borderLayout);

			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ProgressNSliderTestDialog::onClose));
            m_horizontalSBar->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ProgressNSliderTestDialog::onHSlider));
            m_verticalSBar->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(ProgressNSliderTestDialog::onVSlider));
		}

        void ProgressNSliderTestDialog::onHSlider(const Event::MouseEvent &)
		{
            m_horizontalPBar->setValue(m_horizontalSBar->getValue());
			std::ostringstream ostr ;
            ostr<<"Value:"<<static_cast<int>(m_horizontalSBar->getValue())<<"%";
            m_valueLabel->setText(ostr.str());
		}

        void ProgressNSliderTestDialog::onVSlider(const Event::MouseEvent &)
		{
            m_verticalPBar->setValue(m_verticalSBar->getValue());
		}

        void ProgressNSliderTestDialog::onClose(const Event::MouseEvent &)
		{
			Close();
		}

		ProgressNSliderTestDialog::~ProgressNSliderTestDialog(void)
		{
            delete m_closeButton;
            delete m_valueLabel;
            delete m_horizontalPBar;
            delete m_verticalPBar;
            delete m_horizontalSBar;
            delete m_verticalSBar;
            delete m_borderLayout;
            delete m_centerPanel;
            delete m_centerGridLayout;
		}
	}
}
