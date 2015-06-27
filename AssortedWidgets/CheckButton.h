#pragma once
#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class CheckButton:public AbstractButton
		{
		private:
			std::string text;
			bool check;
		public:
			bool isCheck()
			{
				return check;
			};

			void setCheck(bool _check)
			{
				check=_check;
			};

            const std::string& getText() const
			{
				return text;
            }

			void setText(std::string &_text)
			{
				text=_text;
			};

			CheckButton(std::string &_text,bool _check=false);
			CheckButton(char *_text,bool _check=false);
			void mouseReleased(const Event::MouseEvent &e);
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getCheckButtonPreferedSize(this);
			};
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintCheckButton(this);
			};
		public:
			~CheckButton(void);
		};
	}
}
