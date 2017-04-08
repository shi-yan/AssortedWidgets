#include "MultipleLayoutTestDialog.h"

namespace AssortedWidgets
{
	namespace Test
	{
		MultipleLayoutTestDialog::MultipleLayoutTestDialog(void):Dialog("MultipleLayout Test:",350,350,400,180)
		{
            m_gridLayout=new Layout::GridLayout(1,2);
            m_gridLayout->setRight(16);
            m_gridLayout->setLeft(16);
            m_gridLayout->setTop(8);
            m_gridLayout->setBottom(8);
            m_gridLayout->setSpacer(4);

            m_flowLayout=new Layout::FlowLayout(2,2,2,2,4);
			

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

            m_flowPanel=new Widgets::Panel();
            m_flowPanel->setLayout(m_flowLayout);
            m_flowPanel->add(m_TheLabel);
            m_flowPanel->add(m_quickLabel);
            m_flowPanel->add(m_brownLabel);
            m_flowPanel->add(m_foxLabel);
            m_flowPanel->add(m_jumpsLabel);
            m_flowPanel->add(m_overLabel);
            m_flowPanel->add(m_aLabel);
            m_flowPanel->add(m_lazyDogLabel);
		
            m_flowPanel->pack();

            m_borderLayout=new Layout::BorderLayout(2,2,2,2,4);
            m_closeButton=new Widgets::Button("Close");
            m_closeButton->setLayoutProperty(Layout::BorderLayout::South);

            m_northLabel=new Widgets::Label("North");
            m_northLabel->setHorizontalStyle(Widgets::Label::Stretch);
            m_northLabel->setDrawBackground(true);
            m_northLabel->setLayoutProperty(Layout::BorderLayout::North);

            m_southLabel=new Widgets::Label("South");
            m_southLabel->setHorizontalStyle(Widgets::Label::Stretch);
            m_southLabel->setDrawBackground(true);
            m_southLabel->setLayoutProperty(Layout::BorderLayout::South);

            m_westLabel=new Widgets::Label("West");
            m_westLabel->setVerticalStyle(Widgets::Label::Stretch);
            m_westLabel->setDrawBackground(true);
            m_westLabel->setLayoutProperty(Layout::BorderLayout::West);

            m_eastLabel=new Widgets::Label("East");
            m_eastLabel->setVerticalStyle(Widgets::Label::Stretch);
            m_eastLabel->setDrawBackground(true);
            m_eastLabel->setLayoutProperty(Layout::BorderLayout::East);

            m_centerLabel=new Widgets::Label("Center");
            m_centerLabel->setHorizontalStyle(Widgets::Label::Stretch);
            m_centerLabel->setVerticalStyle(Widgets::Label::Stretch);
            m_centerLabel->setDrawBackground(true);
            m_centerLabel->setLayoutProperty(Layout::BorderLayout::Center);

            m_borderPanel=new Widgets::Panel();
            m_borderPanel->setLayout(m_borderLayout);

            m_borderPanel->add(m_northLabel);
            m_borderPanel->add(m_southLabel);
            m_borderPanel->add(m_closeButton);
            m_borderPanel->add(m_westLabel);
            m_borderPanel->add(m_eastLabel);
            m_borderPanel->add(m_centerLabel);

            m_borderPanel->pack();
			
            setLayout(m_gridLayout);
            add(m_flowPanel);
            add(m_borderPanel);
			pack();

            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(MultipleLayoutTestDialog::onClose));
		}

		void MultipleLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		MultipleLayoutTestDialog::~MultipleLayoutTestDialog(void)
		{
            delete m_closeButton;
            delete m_northLabel;
            delete m_southLabel;
            delete m_westLabel;
            delete m_eastLabel;
            delete m_borderLayout;
            delete m_centerLabel;
            delete m_flowLayout;
            delete m_TheLabel;
            delete m_quickLabel;
            delete m_brownLabel;
            delete m_foxLabel;
            delete m_jumpsLabel;
            delete m_overLabel;
            delete m_aLabel;
            delete m_lazyDogLabel;
            delete m_gridLayout;
            delete m_flowPanel;
            delete m_borderPanel;
		}
    }
}
