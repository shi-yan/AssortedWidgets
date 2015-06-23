#pragma once

namespace AssortedWidgets
{
	
	namespace Widgets
	{
		class RadioButton;
		class RadioGroup
		{
		private:
			RadioButton *currentChecked;
		public:
			RadioButton* getChecked()
			{
				return currentChecked;
			};

			void setCheck(RadioButton *_currentChecked);
		
			RadioGroup(void);
		public:
			~RadioGroup(void);
		};
	}
}