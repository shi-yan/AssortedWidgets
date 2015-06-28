#pragma once

#include <vector>
#include "Size.h"
#include "Position.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Element;
	}

	namespace Layout
	{
		class Layout
		{
		protected:
            unsigned int m_top;
            unsigned int m_bottom;
            unsigned int m_left;
            unsigned int m_right;
            unsigned int m_spacer;

        public:
            Layout(unsigned int spacer = 2, unsigned int top = 0, unsigned int bottom = 0, unsigned int left = 0, unsigned int right = 0)
                :m_top(top),
                  m_bottom(bottom),
                  m_left(left),
                  m_right(right),
                  m_spacer(spacer)
            {}

            void setTop(unsigned int top)
			{
                m_top = top;
            }

            void setBottom(unsigned int bottom)
			{
                m_bottom = bottom;
            }

            void setLeft(unsigned int left)
			{
                m_left = left;
            }

            void setRight(unsigned int right)
			{
                m_right = right;
            }

            void setSpacer(unsigned int spacer)
			{
                m_spacer = spacer;
            }

            unsigned int getTop() const
			{
                return m_top;
            }

            unsigned int getBottom() const
			{
                return m_bottom;
            }

            unsigned int getLeft() const
			{
                return m_left;
            }

            unsigned int getRight() const
			{
                return m_right;
            }

            unsigned int getSpacer() const
			{
                return m_spacer;
            }

            virtual void updateLayout(std::vector<Widgets::Element *> &componentList, Util::Position &origin, Util::Size &area) = 0;
            virtual Util::Size getPreferedSize() const = 0;
            virtual void testPaint() {}
		public:
            virtual ~Layout(void) {}
		};
	}
}
