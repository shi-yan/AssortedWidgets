#pragma once
#include "ContainerElement.h"
#include "KeyEvent.h"
#include <ctype.h>
#include <string>

namespace AssortedWidgets
{
	namespace Widgets
	{
        class TypeAble: public Element
		{
		private:
            std::string m_text;
            bool m_active;
		public:
            TypeAble(const std::string &_text = std::string());
			bool isActive()
			{
                return m_active;
            }
            const std::string& getText() const
			{
                return m_text;
			}
			void setActive(bool _active)
			{
                m_active=_active;
            }
			void mousePressed(const Event::MouseEvent &e);
            void onCharTyped(char character,int modifier);

		public:
			~TypeAble(void);
		};
	}
}
