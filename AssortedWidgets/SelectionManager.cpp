#include "SelectionManager.h"
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		SelectionManager::~SelectionManager(void)
		{
			//clear();
		}

		void SelectionManager::registerComponent(Widgets::Component *component)
		{
			int hSIndex(static_cast<int>(static_cast<float>(component->position.x)/static_cast<float>(girdSize)));
			int vSIndex(static_cast<int>(static_cast<float>(component->position.y)/static_cast<float>(girdSize)));
			int hEIndex(static_cast<int>(static_cast<float>(component->position.x+component->size.width)/static_cast<float>(girdSize)));
			int vEIndex(static_cast<int>(static_cast<float>(component->position.y+component->size.height)/static_cast<float>(girdSize)));
			hSIndex=std::max<int>(hSIndex,0);
			vSIndex=std::max<int>(vSIndex,0);
			hEIndex=std::min<int>(hEIndex,horizonalCount-1);
			vEIndex=std::min<int>(vEIndex,verticalCount-1);

			for(int i=hSIndex;i<=hEIndex;++i)
			{
				for(int e=vSIndex;e<=vEIndex;++e)
				{
					girdTable[i][e].push_back(component);
				}
			}
		}

		void SelectionManager::changePosition(Util::Position &oldP,Util::Size &oldS,Widgets::Component *component)
		{
			int ohSIndex(static_cast<int>(static_cast<float>(oldP.x)/static_cast<float>(girdSize)));
			int ovSIndex(static_cast<int>(static_cast<float>(oldP.y)/static_cast<float>(girdSize)));
			int ohEIndex(static_cast<int>(static_cast<float>(oldP.x+oldS.width)/static_cast<float>(girdSize)));
			int ovEIndex(static_cast<int>(static_cast<float>(oldP.y+oldS.height)/static_cast<float>(girdSize)));
			ohSIndex=std::max<int>(ohSIndex,0);
			ovSIndex=std::max<int>(ovSIndex,0);
			ohEIndex=std::min<int>(ohEIndex,horizonalCount-1);
			ovEIndex=std::min<int>(ovEIndex,verticalCount-1);

			for(int i=ohSIndex;i<=ohEIndex;++i)
			{
				for(int e=ovSIndex;e<=ovEIndex;++e)
				{
					girdTable[i][e].erase(std::remove(girdTable[i][e].begin(),girdTable[i][e].end(),component),girdTable[i][e].end());
				}
			}

			int hSIndex(static_cast<int>(static_cast<float>(component->position.x)/static_cast<float>(girdSize)));
			int vSIndex(static_cast<int>(static_cast<float>(component->position.y)/static_cast<float>(girdSize)));
			int hEIndex(static_cast<int>(static_cast<float>(component->position.x+component->size.width)/static_cast<float>(girdSize)));
			int vEIndex(static_cast<int>(static_cast<float>(component->position.y+component->size.height)/static_cast<float>(girdSize)));
			hSIndex=std::max<int>(hSIndex,0);
			vSIndex=std::max<int>(vSIndex,0);
			hEIndex=std::min<int>(hEIndex,horizonalCount-1);
			vEIndex=std::min<int>(vEIndex,verticalCount-1);

			for(int i=hSIndex;i<=hEIndex;++i)
			{
				for(int e=vSIndex;e<=vEIndex;++e)
				{
					girdTable[i][e].push_back(component);
				}
			}
		}
	}
}