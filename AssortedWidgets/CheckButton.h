#pragma once
#include "AbstractButton.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class CheckButton:public AbstractButton
		{
		private:
            std::string m_text;
            bool m_check;
		public:
            bool isCheck() const
			{
                return m_check;
            }

			void setCheck(bool _check)
			{
                m_check=_check;
            }

            const std::string& getText() const
			{
                return m_text;
            }

			void setText(std::string &_text)
			{
                m_text=_text;
            }

            CheckButton(const std::string &_text,bool _check=false);
            CheckButton(const char *_text,bool _check=false);
			void mouseReleased(const Event::MouseEvent &e);
			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getCheckButtonPreferedSize(this);
            }

			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintCheckButton(this);
            }
		public:
			~CheckButton(void);
		};
	}
}
