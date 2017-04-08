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
            std::string m_text;
            bool m_check;
            RadioGroup *m_group;
		public:
            bool isCheck() const
			{
                return m_check;
            }

			void checkOff()
			{
                m_check=false;
            }

			void checkOn()
			{
                m_check=true;
            }

			void mouseReleased(const Event::MouseEvent &e);
            const std::string& getText() const
			{
                return m_text;
            }

            void setText(const std::string &_text)
			{
                m_text=_text;
            }

            RadioButton(const std::string &_text,RadioGroup *_group);

			Util::Size getPreferedSize()
			{
				return Theme::ThemeEngine::getSingleton().getTheme().getRadioButtonPreferedSize(this);
            }
			void paint()
			{
				Theme::ThemeEngine::getSingleton().getTheme().paintRadioButton(this);
            }
		public:
			~RadioButton(void);
		};
	}
}
