#include "TypeActiveManager.h"
#include "TypeAble.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		void TypeActiveManager::setActive(Widgets::TypeAble *_currentActive)
		{
			if(currentActive)
			{
				currentActive->setActive(false);
			}
			currentActive=_currentActive;
		};

		void TypeActiveManager::disactive()
		{
			if(currentActive)
			{
				currentActive->setActive(false);
				currentActive=0;
			}
		};

		void TypeActiveManager::onCharTyped(char character,int modifier)
		{
			if(currentActive)
			{
				currentActive->onCharTyped(character,modifier);
			}
		};

		TypeActiveManager::~TypeActiveManager(void)
		{
		}
	}
}