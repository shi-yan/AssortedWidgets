#pragma once
#include "AbstractButton.h"
#include "RadioGroup.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class RadioButton: public AbstractButton
		{
		private:
			std::string text;
			bool check;
			RadioGroup *group;
		public:
			bool isCheck()
			{
				return check;
			};

			void checkOff()
			{
				check=false;
			};

			void checkOn()
			{
				check=true;
			};
			void mouseReleased(const Event::MouseEvent &e);
            const std::string& getText() const
			{
				return text;
            }

			void setText(std::string &_text)
			{
				text=_text;
			};

			RadioButton(std::string &_text,RadioGroup *_group);
			RadioButton(char *_text,RadioGroup *_group);

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getRadioButtonPreferedSize(this);
			};
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintRadioButton(this);
			};
		public:
			~RadioButton(void);
		};
	}
}
