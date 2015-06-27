#pragma once
#include "ContainerElement.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class SlideBarSlider;

		class SlideBar:public Element
		{
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};

		private:
			SlideBarSlider *slider;
			int type;
			float value;
			float minV;
			float maxV;

		public:
			float getValue()
			{
				return (maxV-minV)*value+minV;
			};
			float getMax()
			{
				return maxV;
			};
			//void onValueChanged();
			void setValue(float _value)
			{
				if(_value>=minV && _value<=maxV)
				{
					value=(_value-minV)/(maxV-minV);
				}
			};
			void setPercent(float _value)
			{
				value=_value;
			};
			SlideBar(int _type=Horizontal);
			SlideBar(float _minV,float _maxV,int _type=Horizontal);
			SlideBar(float _minV,float _maxV,float _value,int _type=Horizontal);
			int getType()
			{
				return type;
			};
			Util::Size getPreferedSize()
			{
				if(type==Horizontal)
				{
					return Util::Size(10,20);
				}
				else if(type==Vertical)
				{
					return Util::Size(20,10);
				}
				return Util::Size();
			};
			void paint();
			void mousePressed(const Event::MouseEvent &e);
			void pack();
		public:
			~SlideBar(void);
		};
	}
}
