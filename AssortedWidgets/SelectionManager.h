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
            std::vector<std::vector<std::vector<Widgets::Component*> > > m_girdTable;
            unsigned int m_girdSize;
            unsigned int m_horizonalCount;
            unsigned int m_verticalCount;
            unsigned int m_width;
            unsigned int m_height;
		public:
            SelectionManager()
                :m_girdSize(32)
			{
            }

		public:
			void setup(unsigned int _width,unsigned int _height)
			{
                m_width=_width;
                m_height=_height;
                m_horizonalCount=m_width/m_girdSize+1;
                m_verticalCount=m_height/m_girdSize+1;
                m_girdTable.reserve(m_horizonalCount);
                for(size_t i=0;i<m_horizonalCount;++i)
				{
                    m_girdTable.push_back(std::vector<std::vector<Widgets::Component*> >());
                    m_girdTable[i].reserve(m_verticalCount);
                    for(size_t e=0;e<m_verticalCount;++e)
					{
                        m_girdTable[i].push_back(std::vector<Widgets::Component*>());
                        m_girdTable[i][e].reserve(10);
					}
				}
            }

			void clear()
			{
                for(size_t i=0;i<m_horizonalCount;++i)
				{
                    for(size_t e=0;e<m_verticalCount;++e)
					{
                        m_girdTable[i][e].clear();
					}
                    m_girdTable[i].clear();
				}
                m_girdTable.clear();
                m_width=0;
                m_height=0;
                m_horizonalCount=0;
                m_verticalCount=0;
			}

			void registerComponent(Widgets::Component *component);

			std::vector<Widgets::Component*>& getHitComponents(int x,int y)
			{
                int h(static_cast<int>(static_cast<float>(x)/static_cast<float>(m_girdSize)));
                int v(static_cast<int>(static_cast<float>(y)/static_cast<float>(m_girdSize)));
                return m_girdTable[h][v];
            }

			bool testHit(int x,int y,Widgets::Component *component)
			{
                int h(static_cast<int>(static_cast<float>(x)/static_cast<float>(m_girdSize)));
                int v(static_cast<int>(static_cast<float>(y)/static_cast<float>(m_girdSize)));
				std::vector<Widgets::Component*>::iterator iter;
                for(iter=m_girdTable[h][v].begin();iter<m_girdTable[h][v].end();++iter)
				{
					if((*iter)==component)
					{
						return true;
					}
				}
				return false;
            }

			void changePosition(Util::Position &oldP,Util::Size &oldS,Widgets::Component *component);

		public:
			~SelectionManager(void);
		};
	}
}
