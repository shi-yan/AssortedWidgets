#pragma once

#include <vector>
#include <algorithm>
//#include "ContainerElement.h"
#include "Position.h"
#include "Size.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Component;
	}

	namespace Manager
	{
		class SelectionManager
		{
		private:
			std::vector<std::vector<std::vector<Widgets::Component*>>> girdTable;
			unsigned int girdSize;
			unsigned int horizonalCount;
			unsigned int verticalCount;
			unsigned int width;
			unsigned int height;
		public:
			SelectionManager():girdSize(32)
			{
			};

		public:
			void setup(unsigned int _width,unsigned int _height)
			{
				width=_width;
				height=_height;
				horizonalCount=width/girdSize+1;
				verticalCount=height/girdSize+1;
				girdTable.reserve(horizonalCount);
				for(size_t i=0;i<horizonalCount;++i)
				{
					girdTable.push_back(std::vector<std::vector<Widgets::Component*>>());
					girdTable[i].reserve(verticalCount);
					for(size_t e=0;e<verticalCount;++e)
					{
						girdTable[i].push_back(std::vector<Widgets::Component*>());
						girdTable[i][e].reserve(10);
					}
				}
			};

			void clear()
			{
				for(size_t i=0;i<horizonalCount;++i)
				{
					for(size_t e=0;e<verticalCount;++e)
					{
						girdTable[i][e].clear();
					}
					girdTable[i].clear();
				}
				girdTable.clear();
				width=0;
				height=0;
				horizonalCount=0;
				verticalCount=0;
			}

			void registerComponent(Widgets::Component *component);

			std::vector<Widgets::Component*>& getHitComponents(int x,int y)
			{
				int h(static_cast<int>(static_cast<float>(x)/static_cast<float>(girdSize)));
				int v(static_cast<int>(static_cast<float>(y)/static_cast<float>(girdSize)));
				return girdTable[h][v];
			};

			bool testHit(int x,int y,Widgets::Component *component)
			{
				int h(static_cast<int>(static_cast<float>(x)/static_cast<float>(girdSize)));
				int v(static_cast<int>(static_cast<float>(y)/static_cast<float>(girdSize)));				
				std::vector<Widgets::Component*>::iterator iter;
				for(iter=girdTable[h][v].begin();iter<girdTable[h][v].end();++iter)
				{
					if((*iter)==component)
					{
						return true;
					}
				}
				return false;
			};

			void changePosition(Util::Position &oldP,Util::Size &oldS,Widgets::Component *component);

		public:
			~SelectionManager(void);
		};
	}
}