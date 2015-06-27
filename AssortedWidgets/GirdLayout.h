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
				int HAlignment;
				int VAlignment;
				Widgets::Element *component;
				unsigned int width;
				unsigned int height;
				int HStyle;
				int VStyle;
			};
		
			struct Alignment **alignment;
			unsigned int rowCount;
			unsigned int columnCount;

		public:
			GirdLayout(unsigned int _rowCount,unsigned int _columnCount):Layout(),rowCount(_rowCount),columnCount(_columnCount)
			{
				alignment=new struct Alignment*[rowCount];
                for(unsigned int i=0;i<rowCount;++i)
				{
						alignment[i]=new struct Alignment[columnCount];
                        for(unsigned int e=0;e<columnCount;++e)
						{
							alignment[i][e].HAlignment=HLeft;
							alignment[i][e].VAlignment=VTop;
						}
				}
			};

			void updateLayout(std::vector<Widgets::Element *> &componentList,Util::Position &origin,Util::Size &area);
            Util::Size getPreferedSize() const;

			void setHorizontalAlignment(unsigned int i,unsigned int e,int _HAlignment)
			{
				if(i<rowCount && e<columnCount)
				{
					alignment[i][e].HAlignment=_HAlignment;
				}
			};

			void setVerticalAlignment(unsigned int i,unsigned int e,int _VAlignment)
			{
				if(i<rowCount && e<columnCount)
				{
					alignment[i][e].VAlignment=_VAlignment;
				}
			};

		private:
			void orderComponent(unsigned int row,unsigned int column,Util::Position &origin,Util::Size &size);
		public:
			~GirdLayout(void);
		};
	}
}
