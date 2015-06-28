#pragma once

#include "Component.h"

namespace AssortedWidgets
{
	class Widgets::Component;

	namespace Event
	{
		class Event
		{
		private:
            Widgets::Component *m_source;
            int m_type;
		public:
            Event(Widgets::Component *_source,int _type)
                :m_source(_source),
                  m_type(_type)
            {}

            Widgets::Component* getSource() const
			{
                return m_source;
            }

			int getType() const
			{
                return m_type;
            }
		public:
            ~Event(void){}
		};
	}
}
