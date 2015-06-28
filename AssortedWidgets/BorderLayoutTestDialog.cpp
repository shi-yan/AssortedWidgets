#include "BorderLayoutTestDialog.h"
namespace AssortedWidgets
{
	namespace Test
	{
		BorderLayoutTestDialog::BorderLayoutTestDialog(void):Dialog("BorderLayout Test:",250,250,320,240)
		{
            m_borderLayout=new Layout::BorderLayout(4, 16, 16, 16, 16);
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

            setLayout(m_borderLayout);

            add(m_northLabel);
            add(m_southLabel);
            add(m_closeButton);
            add(m_westLabel);
            add(m_eastLabel);
            add(m_centerLabel);

			pack();
            m_closeButton->mouseReleasedHandlerList.push_back(MOUSE_DELEGATE(BorderLayoutTestDialog::onClose));
		}

				void BorderLayoutTestDialog::onClose(const Event::MouseEvent &e)
		{
			Close();
		}

		BorderLayoutTestDialog::~BorderLayoutTestDialog(void)
		{
            delete m_closeButton;
            delete m_northLabel;
            delete m_southLabel;
            delete m_westLabel;
            delete m_eastLabel;
            delete m_borderLayout;
            delete m_centerLabel;
		}
	}
}
