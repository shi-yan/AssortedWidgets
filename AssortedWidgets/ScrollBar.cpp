#include "ScrollBar.h"
#include "ScrollBarButton.h"
#include "ScrollBarSlider.h"
#include "Graphics.h"
#include "ThemeEngine.h"
#include "ScrollPanel.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		ScrollBar::ScrollBar(int _type):type(_type),value(0)
		{
			if(type==Horizontal)
			{
				min=new ScrollBarButton(ScrollBarButton::HorizontalLeft);
				max=new ScrollBarButton(ScrollBarButton::HorizontalRight);
				slider=new ScrollBarSlider(ScrollBarSlider::Horizontal);
				setHorizontalStyle(Element::Stretch);
				setVerticalStyle(Element::Fit);
				size.width=40;
				size.height=15;
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-30)*0.1f),4);
				slider->size.height=11;
				slider->position.x=static_cast<int>(((size.width-34)-slider->size.width)*value+17);
				slider->position.y=2;
				slider->setScrollBar(this);
			}
			else if(type==Vertical)
			{
				min=new ScrollBarButton(ScrollBarButton::VerticalTop);
				max=new ScrollBarButton(ScrollBarButton::VerticalBottom);
				slider=new ScrollBarSlider(ScrollBarSlider::Vertical);
				setHorizontalStyle(Element::Fit);
				setVerticalStyle(Element::Stretch);
				size.width=15;
				size.height=40;
				slider->size.width=11;
				slider->size.height=std::max<unsigned int>(static_cast<unsigned int>((size.height-30)*0.1),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-34)-slider->size.height)*value+17);
				slider->setScrollBar(this);
			}

			MouseDelegate mPressed;
			mPressed.bind(this,&ScrollBar::mousePressed);
			mousePressedHandlerList.push_back(mPressed);

			MouseDelegate mReleased;
			mReleased.bind(this,&ScrollBar::mouseReleased);
			mouseReleasedHandlerList.push_back(mReleased);

			MouseDelegate mEntered;
			mEntered.bind(this,&ScrollBar::mouseEntered);
			mouseEnteredHandlerList.push_back(mEntered);
			
			MouseDelegate mExited;
			mExited.bind(this,&ScrollBar::mouseExited);
			mouseExitedHandlerList.push_back(mExited);

			MouseDelegate mMoved;
			mMoved.bind(this,&ScrollBar::mouseMoved);
			mouseMovedHandlerList.push_back(mMoved);
			

			MouseDelegate minReleased;
			minReleased.bind(this,&ScrollBar::onMinReleased);
			min->mouseReleasedHandlerList.push_back(minReleased);

			MouseDelegate maxReleased;
			maxReleased.bind(this,&ScrollBar::onMaxReleased);
			max->mouseReleasedHandlerList.push_back(maxReleased);
		}

		void ScrollBar::mouseMoved(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(min->isIn(mx,my))
			{
				if(!min->isHover)
				{
					Event::MouseEvent event(min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					min->processMouseEntered(event);
				}
			}
			else
			{
				if(min->isHover)
				{
					Event::MouseEvent event(min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					min->processMouseExited(event);
				}
			}

			if(max->isIn(mx,my))
			{
				if(!max->isHover)
				{
					Event::MouseEvent event(max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					max->processMouseEntered(event);
				}
			}	
			else
			{
				if(max->isHover)
				{
					Event::MouseEvent event(max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					max->processMouseExited(event);
				}
			}
		}

		void ScrollBar::mouseEntered(const Event::MouseEvent &e)
		{
			isHover=true;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
				min->processMouseEntered(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
				max->processMouseEntered(event);
				return;			
			}
		}

		void ScrollBar::mouseExited(const Event::MouseEvent &e)
		{
			isHover=false;
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(min->isHover)
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
				min->processMouseExited(event);
				return;
			}
			else if(max->isHover)
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
				max->processMouseExited(event);
				return;			
			}
		}

		void ScrollBar::mouseReleased(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				min->processMouseReleased(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				max->processMouseReleased(event);
				return;			
			}
		}

		void ScrollBar::onMinReleased(const Event::MouseEvent &e)
		{
			value=std::max<float>(value-0.1f,0.0f);
			if(type==Horizontal)
			{
				slider->position.x=static_cast<int>(((size.width-34)-slider->size.width)*value+17);
				slider->position.y=2;
			}
			else if(type==Vertical)
			{
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-34)-slider->size.height)*value+17);
			}
			onValueChanged();
		}

		void ScrollBar::onMaxReleased(const Event::MouseEvent &e)
		{
			value=std::min<float>(value+0.1f,1.0f);
			if(type==Horizontal)
			{
				slider->position.x=static_cast<int>(((size.width-34)-slider->size.width)*value+17);
				slider->position.y=2;
			}
			else if(type==Vertical)
			{
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-34)-slider->size.height)*value+17);
			}
			onValueChanged();
		}

		void ScrollBar::onValueChanged()
		{
			parent->onValueChanged(this);
		};

		void ScrollBar::mousePressed(const Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;
			if(slider->isIn(mx,my))
			{
				Event::MouseEvent event(slider,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
				slider->processMousePressed(event);
				return;
			}
			else if(min->isIn(mx,my))
			{
				Event::MouseEvent event(min,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				min->processMousePressed(event);
				return;
			}
			else if(max->isIn(mx,my))
			{
				Event::MouseEvent event(max,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
				max->processMousePressed(event);
				return;			
			}
		}

		void ScrollBar::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintScrollBar(this);
			Util::Graphics::getSingleton().pushPosition(Util::Position(position));
			min->paint();
			max->paint();
			slider->paint();
			Util::Graphics::getSingleton().popPosition();
		};

		void ScrollBar::pack()
		{
			if(type==Horizontal)
			{
				min->position.x=0;
				min->position.y=0;
				max->position.x=size.width-15;
				max->position.y=0;
				slider->size.width=std::max<unsigned int>(static_cast<unsigned int>((size.width-30)*0.1f),4);
				slider->size.height=11;
				slider->position.x=static_cast<int>(((size.width-34)-slider->size.width)*value+17);
				slider->position.y=2;
			}
			else if(type==Vertical)
			{
				min->position.x=0;
				min->position.y=0;
				max->position.x=0;
				max->position.y=size.height-15;
				slider->size.width=11;
				slider->size.height=std::max<unsigned int>(static_cast<unsigned int>((size.height-30)*0.1f),4);
				slider->position.x=2;
				slider->position.y=static_cast<int>(((size.height-34)-slider->size.height)*value+17);		
			}
		};

		ScrollBar::~ScrollBar(void)
		{
			delete slider;
			delete min;
			delete max;
		}
	}
}