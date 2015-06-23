#include "ProgressBar.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		ProgressBar::ProgressBar(void):type(Horizontal),min(0.0f),max(100.0f),value(0.0f),POfSlider(0)
		{
			size=getPreferedSize();
			if(type==Horizontal)
			{
				horizontalStyle=Element::Stretch;
				verticalStyle=Element::Fit;
			}
			else if(type==Vertical)
			{
				horizontalStyle=Element::Fit;
				verticalStyle=Element::Stretch;
			}
			pack();
		}

		ProgressBar::ProgressBar(int _type):type(_type),min(0.0f),max(100.0f),value(0.0f),POfSlider(0)
		{
			size=getPreferedSize();
			if(type==Horizontal)
			{
				horizontalStyle=Element::Stretch;
				verticalStyle=Element::Fit;
			}
			else if(type==Vertical)
			{
				horizontalStyle=Element::Fit;
				verticalStyle=Element::Stretch;
			}
			pack();
		}

		ProgressBar::ProgressBar(float _min,float _max,int _type):type(_type),min(_min),max(_max),value(0.0f),POfSlider(0)
		{
			size=getPreferedSize();
			if(type==Horizontal)
			{
				horizontalStyle=Element::Stretch;
				verticalStyle=Element::Fit;
			}
			else if(type==Vertical)
			{
				horizontalStyle=Element::Fit;
				verticalStyle=Element::Stretch;
			}
			pack();
		}

		ProgressBar::ProgressBar(float _min,float _max,float _value,int _type):type(_type),min(_min),max(_max),value(0),POfSlider(0)
		{
			setValue(_value);
			size=getPreferedSize();
			if(type==Horizontal)
			{
				horizontalStyle=Element::Stretch;
				verticalStyle=Element::Fit;
			}
			else if(type==Vertical)
			{
				horizontalStyle=Element::Fit;
				verticalStyle=Element::Stretch;
			}
			pack();
		}

		ProgressBar::~ProgressBar(void)
		{
		}

		void ProgressBar::pack()
		{
			if(type==Horizontal)
			{
				POfSlider=static_cast<int>(value*size.width);
			}
			else if(type==Vertical)
			{
				POfSlider=static_cast<int>(value*size.height);
			}			
		}
	}
}