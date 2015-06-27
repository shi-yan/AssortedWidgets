#include "TypeActiveManager.h"
#include "TypeAble.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		void TypeActiveManager::setActive(Widgets::TypeAble *_currentActive)
		{
            if(m_currentActive)
			{
                m_currentActive->setActive(false);
			}
            m_currentActive=_currentActive;
        }

		void TypeActiveManager::disactive()
		{
            if(m_currentActive)
			{
                m_currentActive->setActive(false);
                m_currentActive=0;
			}
        }

		void TypeActiveManager::onCharTyped(char character,int modifier)
		{
            if(m_currentActive)
			{
                m_currentActive->onCharTyped(character,modifier);
			}
        }

		TypeActiveManager::~TypeActiveManager(void)
		{
		}
	}
}
