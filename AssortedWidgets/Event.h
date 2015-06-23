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
			Widgets::Component *source;
			int type;
		public:
			Event(Widgets::Component *_source,int _type):source(_source),type(_type)
			{};
			Widgets::Component* getSource() const
			{
				return source;
			};
			int getType() const
			{
				return type;
			};
		public:
			~Event(void){};
		};
	}
}