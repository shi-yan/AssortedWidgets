#pragma once

namespace AssortedWidgets
{
	
	namespace Widgets
	{
		class RadioButton;
		class RadioGroup
		{
		private:
            RadioButton *m_currentChecked;
		public:
			RadioButton* getChecked()
			{
                return m_currentChecked;
            }

			void setCheck(RadioButton *_currentChecked);
		
			RadioGroup(void);
		public:
			~RadioGroup(void);
		};
	}
}
