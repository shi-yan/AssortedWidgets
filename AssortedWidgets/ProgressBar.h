#pragma once
#include "ContainerElement.h"
#include "ThemeEngine.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class ProgressBar:public Element
		{
		public:
			enum Type
			{
				Horizontal,
				Vertical
			};
		private:
			int type;
			float value;
			float min;
			float max;
			unsigned int POfSlider;
		public:
			int getType()
			{
				return type;
			};
			float getValue()
			{
				return min+(max-min)*value;
			};

			unsigned int getPOfSlider()
			{
				return POfSlider;	
			};

			void setValue(float _value)
			{
				if(_value>=min && _value<=max)
				{
					value=(_value-min)/(max-min);
					if(type==Horizontal)
					{
						POfSlider=static_cast<unsigned int>(value*size.width);
					}
					else if(type==Vertical)
					{
						POfSlider=static_cast<unsigned int>(value*size.height);
					}
				}
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
			
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintProgressBar(this);
			};

			void pack();
			ProgressBar(void);
			ProgressBar(int _type);
			ProgressBar(float _min,float _max,int _type=Horizontal);
			ProgressBar(float _min,float _max,float _value,int _type=Horizontal);
		public:
			~ProgressBar(void);
		};
	}
}