#pragma once

namespace AssortedWidgets
{
	namespace Widgets
	{
		class TypeAble;
	}

	namespace Manager
	{
		class TypeActiveManager
		{
		private:
			Widgets::TypeAble *currentActive;
		private:
			TypeActiveManager(void):currentActive(0)
			{};
		public:
			void setActive(Widgets::TypeAble *_currentActive);
			void onCharTyped(char character,int modifier);
			bool isActive()
			{
				return currentActive!=0;
			};
			void disactive();
			static TypeActiveManager& getSingleton()
			{
				static TypeActiveManager obj;
				return obj;
			}
		private:
			~TypeActiveManager(void);
		};
	}
}