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
            Widgets::TypeAble *m_currentActive;
            TypeActiveManager(void)
                :m_currentActive(0)
            {}
            ~TypeActiveManager(void);
		public:
			void setActive(Widgets::TypeAble *_currentActive);
			void onCharTyped(char character,int modifier);
			bool isActive()
			{
                return m_currentActive!=0;
            }
			void disactive();
			static TypeActiveManager& getSingleton()
			{
				static TypeActiveManager obj;
				return obj;
			}
		};
	}
}
