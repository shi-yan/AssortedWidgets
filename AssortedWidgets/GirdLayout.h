#pragma once
#include "Layout.h"
#include <vector>

namespace AssortedWidgets
{
	namespace Layout
	{
		class GirdLayout:public Layout
		{
		public:
			enum HorizontalAlignment
			{
				HLeft,
				HCenter,
				HRight
			};

			enum VerticalAlignment
			{
				VTop,
				VCenter,
				VBottom
			};
		private:
			struct Alignment
			{
                int m_HAlignment;
                int m_VAlignment;
                Widgets::Element *m_component;
                unsigned int m_width;
                unsigned int m_height;
                int m_HStyle;
                int m_VStyle;
			};
		
            struct Alignment **m_alignment;
            unsigned int m_rowCount;
            unsigned int m_columnCount;

		public:
            GirdLayout(unsigned int _rowCount,unsigned int _columnCount)
                :Layout(),
                  m_rowCount(_rowCount),
                  m_columnCount(_columnCount)
			{
                m_alignment=new struct Alignment*[m_rowCount];
                for(unsigned int i=0;i<m_rowCount;++i)
				{
                        m_alignment[i]=new struct Alignment[m_columnCount];
                        for(unsigned int e=0; e<m_columnCount; ++e)
						{
                            m_alignment[i][e].m_HAlignment=HLeft;
                            m_alignment[i][e].m_VAlignment=VTop;
						}
				}
            }

			void updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area);
            Util::Size getPreferedSize() const;

			void setHorizontalAlignment(unsigned int i,unsigned int e,int _HAlignment)
			{
                if(i<m_rowCount && e<m_columnCount)
				{
                    m_alignment[i][e].m_HAlignment=_HAlignment;
				}
            }

			void setVerticalAlignment(unsigned int i,unsigned int e,int _VAlignment)
			{
                if(i<m_rowCount && e<m_columnCount)
				{
                    m_alignment[i][e].m_VAlignment=_VAlignment;
				}
            }

		private:
			void orderComponent(unsigned int row,unsigned int column,Util::Position &origin,Util::Size &size);
		public:
			~GirdLayout(void);
		};
	}
}
